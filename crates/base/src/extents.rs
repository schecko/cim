use crate::point::Point;

use arrayvec::ArrayVec;
use bitflags::bitflags;

bitflags!
{
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Neighbours: u8
    {
        const None = 0 << 0;
        const Top = 1 << 0;
        const Bottom = 1 << 1;
        const Left = 1 << 2;
        const Right = 1 << 3;
        const TopLeft = 1 << 4;
        const TopRight = 1 << 5;
        const BottomLeft = 1 << 6;
        const BottomRight = 1 << 7;

        const Vertical = Self::Top.bits() | Self::Bottom.bits();
        const Horizontal = Self::Right.bits() | Self::Left.bits();
        const Flush = Self::Vertical.bits() | Self::Horizontal.bits();
        const Diagonal = Self::TopLeft.bits() | Self::TopRight.bits() | Self::BottomLeft.bits() | Self::BottomRight.bits();
        const All = Self::Flush.bits() | Self::Diagonal.bits();
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Extents
{
    pub width: i32,
    pub height: i32,
}

#[allow(dead_code)]
impl Extents
{
    pub fn new(width: i32, height: i32) -> Self
    {
        assert!(width > 0 && height > 0);
        Extents{ width, height }
    }

    pub fn as_vec2(&self) -> glam::Vec2
    {
        glam::Vec2::new(self.width as f32, self.height as f32)
    }

    pub fn num_elements(&self) -> usize
    {
        (self.width * self.height) as usize
    }

    pub fn is_valid_pos(&self, pos: Point) -> bool
    {
        return pos.y >= 0 && pos.y < self.height && pos.x >= 0 && pos.x < self.width;
    }

    pub fn get_index(&self, pos: Point) -> Option<usize>
    {
        if self.is_valid_pos(pos)
        {
            let index = (pos.y * self.width + pos.x) as usize;
            assert!(index < (self.width * self.height) as usize);
            Some(index)
        }
        else
        {
            None
        }
    }

    pub fn get_index2(&self, index: usize) -> Option<Point>
    {
        if index < self.num_elements()
        {
            Some((index as i32 % self.width, index as i32 / self.width).into())
        }
        else
        {
            None
        }
    }

    pub fn index_space(
        self,
    ) -> impl DoubleEndedIterator<Item = usize> + Clone
    {
        0..self.num_elements()
    }

    pub fn index2_space(
        self,
    ) -> impl DoubleEndedIterator<Item = Point> + Clone + use<>
    {
        (0..self.height).flat_map(move |y| (0..self.width).map(move |x| (x, y).into()))
    }
    
    pub fn neighbours_self<const FLAGS: u8>(
        &self,
        pos: Point,
    ) -> impl DoubleEndedIterator<Item = Point> + Clone + use<FLAGS>
    {
        let mut result = ArrayVec::<Point, 9>::new();
        if self.is_valid_pos(pos)
        {
            result.push(pos);
        }
        for n in self.neighbours::<FLAGS>(pos)
        {
            result.push(n);
        }
        result.into_iter()
    }

    pub fn neighbours<const FLAGS: u8>(
        &self,
        pos: Point,
    ) -> impl DoubleEndedIterator<Item = Point> + Clone + use<FLAGS>
    {
        let mut neigh = ArrayVec::<Point, 8>::new();

        let can_add = |neighbour_position: Neighbours| -> bool
        {
            Neighbours::from_bits_retain( FLAGS ) & neighbour_position != Neighbours::None
        };

        let mut try_add = |pos: Point|
        {
            if self.is_valid_pos(pos)
            {
                neigh.push(pos);
            }
        };

        if can_add(Neighbours::TopLeft) { try_add( ( pos.x.wrapping_sub(1), pos.y.wrapping_sub(1) ).into() ); }
        if can_add(Neighbours::Top) { try_add( ( pos.x, pos.y.wrapping_sub(1) ).into() ); }
        if can_add(Neighbours::TopRight) { try_add( ( pos.x.wrapping_add(1), pos.y.wrapping_sub(1) ).into() ); }

        if can_add(Neighbours::Left) { try_add( ( pos.x.wrapping_sub(1), pos.y ).into() ); }
        if can_add(Neighbours::Right) { try_add( ( pos.x.wrapping_add(1), pos.y ).into() ); }

        if can_add(Neighbours::BottomLeft) { try_add( ( pos.x.wrapping_sub(1), pos.y.wrapping_add(1) ).into() ); }
        if can_add(Neighbours::Bottom) { try_add( ( pos.x, pos.y.wrapping_add(1) ).into() ); }
        if can_add(Neighbours::BottomRight) { try_add( ( pos.x.wrapping_add(1), pos.y.wrapping_add(1) ).into() ); }

        neigh.into_iter()
    }
}

impl From<(i32, i32)> for Extents
{
    fn from(tuple: (i32, i32)) -> Self
    {
        Self {
            width: tuple.0,
            height: tuple.1,
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    fn check_iterators(
        a: impl DoubleEndedIterator<Item = Point> + Clone,
        b: impl DoubleEndedIterator<Item = Point> + Clone
    )
    {
        assert_eq!(a.clone().count(), b.clone().count());

        a.zip(b).for_each(|(a, e)|
        {
            assert_eq!(a, e);
        });
    }

    #[test]
    fn test_neighbours()
    {
        let size = Extents::new( 3, 3 );
        check_iterators(
            size.neighbours::<{ Neighbours::All.bits() }>((1, 1).into()),
            crate::convert::<Point, _, 8>(&[(0, 0), (1, 0), (2, 0), (0, 1), (2, 1), (0, 2), (1, 2), (2, 2),]).into_iter()
        );

        check_iterators(
            size.neighbours::<{ Neighbours::Flush.bits() }>((1, 1).into()),
            crate::convert::<Point, _, 4>(&[ (1, 0), (0, 1), (2, 1), (1, 2) ]).into_iter()
        );
    }

    #[test]
    fn test_neighbours_wrapping()
    {
        let size = Extents::new( 2, 2 );
        check_iterators(
            size.neighbours::<{ Neighbours::All.bits() }>((0, 0).into()),
            crate::convert::<Point, _, 3>(&[ (1, 0), (0, 1), (1, 1) ]).into_iter()
        );

        check_iterators(
            size.neighbours::<{ Neighbours::All.bits() }>((1, 1).into()),
            crate::convert::<Point, _, 3>(&[(0, 0), (1, 0), (0, 1) ]).into_iter()
        );
    }
}


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

pub type Point = glam::IVec2;

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

    pub fn num_elements(&self) -> usize
    {
        (self.width * self.height) as usize
    }

    pub fn is_valid_pos(&self, pos: Point) -> bool
    {
        return pos.y >= 0 && pos.y < self.height && pos.x >= 0 && pos.x < self.width;
    }

    pub fn get_index_row_major(&self, pos: Point) -> Option<usize>
    {
        if self.is_valid_pos(pos)
        {
            Some((pos.y * self.width + pos.x) as usize)
        }
        else
        {
            None
        }
    }

    pub fn indices_row_major(
        self,
    ) -> impl DoubleEndedIterator<Item = usize> + Clone
    {
        0..self.num_elements()
    }

    pub fn positions_row_major(
        self,
    ) -> impl DoubleEndedIterator<Item = Point> + Clone
    {
        (0..self.height).flat_map(move |y| (0..self.width).map(move |x| (x, y).into()))
    }

    pub fn positions_column_major(
        self,
    ) -> impl DoubleEndedIterator<Item = Point> + Clone
    {
        (0..self.width).flat_map(move |x| (0..self.height).map(move |y| (x, y).into()))
    }

    pub fn neighbours<const FLAGS: u8>(
        &self,
        pos: Point,
    ) -> impl DoubleEndedIterator<Item = Point> + Clone
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
    #[should_panic]
    fn test_check_iterators_fail_compare()
    {
        let size = Extents::new( 2, 2 );
        check_iterators(
            size.positions_column_major(),
            [(0_i32, 0_i32), (1, 0), (1, 0), (1, 1)].into_iter() // wrong pos
        );
    }

    #[test]
    #[should_panic]
    fn test_check_iterators_fail_size()
    {
        let size = Extents::new( 2, 2 );
        check_iterators(
            size.positions_column_major(),
            [(0_i32, 0_i32), (1, 1)].into_iter() // too short
        );
    }


    #[test]
    fn test_positions_row_major()
    {
        let size = Extents::new( 2, 2 );
        check_iterators(
            size.positions_row_major(),
            [(0_i32, 0_i32), (1, 0), (0, 1), (1, 1)].into_iter()
        );
    }

    #[test]
    fn test_positions_column_major()
    {
        let size = Extents::new( 2, 2 );
        check_iterators(
            size.positions_column_major(),
            [(0_i32, 0_i32), (0, 1), (1, 0), (1, 1)].into_iter()
        );
    }

    #[test]
    fn test_neighbours()
    {
        let size = Extents::new( 3, 3 );
        check_iterators(
            size.neighbours::<{ Neighbours::All.bits() }>(1, 1),
            [
                (0, 0), (1, 0), (2, 0), (0, 1), (2, 1), (0, 2), (1, 2), (2, 2),
            ].into_iter()
        );

        check_iterators(
            size.neighbours::<{ Neighbours::Flush.bits() }>(1, 1),
            [
                (1, 0), (0, 1), (2, 1), (1, 2)
            ].into_iter()
        );
    }

    #[test]
    fn test_neighbours_wrapping()
    {
        let size = Extents::new( 2, 2 );
        check_iterators(
            size.neighbours::<{ Neighbours::All.bits() }>(0, 0),
            [
                (1, 0), (0, 1), (1, 1)
            ].into_iter()
        );

        check_iterators(
            size.neighbours::<{ Neighbours::All.bits() }>(1, 1),
            [
                (0, 0), (1, 0), (0, 1)
            ].into_iter()
        );
    }
}

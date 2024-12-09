
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
    pub width: usize,
    pub height: usize,
}

#[allow(dead_code)]
impl Extents
{
    pub fn new(width: usize, height: usize) -> Self
    {
        Extents{ width, height }
    }

    pub fn num_elements(&self) -> usize
    {
        self.width * self.height
    }

    pub fn is_valid_pos(&self, x: usize, y: usize) -> bool
    {
        return y < self.height && x < self.width;
    }

    pub fn get_index_row_major(&self, x: usize, y: usize) -> Option<usize>
    {
        if self.is_valid_pos(x, y)
        {
            Some(y * self.width + x)
        }
        else
        {
            None
        }
    }

    pub fn indices_row_major(
        &self,
    ) -> impl DoubleEndedIterator<Item = (usize, usize)> + Clone
    {
        let width = self.width;
        (0..self.height).flat_map(move |y| (0..width).map(move |x| (x, y)))
    }

    pub fn indices_column_major(
        &self,
    ) -> impl DoubleEndedIterator<Item = (usize, usize)> + Clone
    {
        let height = self.height;
        (0..self.width).flat_map(move |x| (0..height).map(move |y| (x, y)))
    }

    pub fn neighbours<const FLAGS: u8>(
        &self,
        x: usize,
        y: usize,
    ) -> impl DoubleEndedIterator<Item = (usize, usize)> + Clone
    {
        let mut neigh = ArrayVec::<(usize, usize), 8>::new();

        if Neighbours::from_bits_retain( FLAGS ) & Neighbours::TopLeft != Neighbours::None && self.is_valid_pos( x - 1, y - 1 ) { neigh.push( (x - 1, y - 1) ); }
        if Neighbours::from_bits_retain( FLAGS ) & Neighbours::Top != Neighbours::None && self.is_valid_pos( x, y - 1 ) { neigh.push( (x, y - 1) ); }
        if Neighbours::from_bits_retain( FLAGS ) & Neighbours::TopRight != Neighbours::None && self.is_valid_pos( x + 1, y - 1 ) { neigh.push( (x + 1, y - 1) ); }

        if Neighbours::from_bits_retain( FLAGS ) & Neighbours::Left != Neighbours::None && self.is_valid_pos( x - 1, y ) { neigh.push( (x - 1, y) ); }
        if Neighbours::from_bits_retain( FLAGS ) & Neighbours::Right != Neighbours::None && self.is_valid_pos( x + 1, y ) { neigh.push( (x + 1, y) ); }

        if Neighbours::from_bits_retain( FLAGS ) & Neighbours::BottomLeft != Neighbours::None && self.is_valid_pos( x - 1, y + 1 ) { neigh.push( (x - 1, y + 1) ); }
        if Neighbours::from_bits_retain( FLAGS ) & Neighbours::Bottom != Neighbours::None && self.is_valid_pos( x, y + 1 ) { neigh.push( (x, y + 1) ); }
        if Neighbours::from_bits_retain( FLAGS ) & Neighbours::BottomRight != Neighbours::None && self.is_valid_pos( x + 1, y + 1 ) { neigh.push( (x + 1, y + 1) ); }

        neigh.into_iter()
    }
}

impl From<(usize, usize)> for Extents
{
    fn from(tuple: (usize, usize)) -> Self
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
        a: impl DoubleEndedIterator<Item = (usize, usize)> + Clone,
        b: impl DoubleEndedIterator<Item = (usize, usize)> + Clone
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
            size.indices_column_major(),
            [(0_usize, 0_usize), (1, 0), (1, 0), (1, 1)].into_iter() // wrong pos
        );
    }

    #[test]
    #[should_panic]
    fn test_check_iterators_fail_size()
    {
        let size = Extents::new( 2, 2 );
        check_iterators(
            size.indices_column_major(),
            [(0_usize, 0_usize), (1, 1)].into_iter() // too short
        );
    }


    #[test]
    fn test_indices_row_major()
    {
        let size = Extents::new( 2, 2 );
        check_iterators(
            size.indices_row_major(),
            [(0_usize, 0_usize), (1, 0), (0, 1), (1, 1)].into_iter()
        );
    }

    #[test]
    fn test_indices_column_major()
    {
        let size = Extents::new( 2, 2 );
        check_iterators(
            size.indices_column_major(),
            [(0_usize, 0_usize), (0, 1), (1, 0), (1, 1)].into_iter()
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
}


use base::array2;
use base::extents;

use bitflags::bitflags;

bitflags!
{
    #[repr(transparent)]
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CellState: u8
    {
        const None = 0 << 0;
        const Mine = 1 << 0;
        const Revealed = 1 << 1;
        const NonPlayable = 1 << 2;
    }
}

#[derive(Debug, Clone)]
pub struct Grid
{
    pub states: array2::Array2<CellState>,
    pub adjacencies: array2::Array2<u8>,
}

impl Grid
{
    pub fn new(width: i32, height: i32) -> Self
    {
        Self
        {
            states: array2::Array2::new(width, height),
            adjacencies: array2::Array2::new(width, height),
        }
    }

    pub fn from_size(size: extents::Extents) -> Self
    {
        Self
        {
            states: array2::Array2::from_size(size),
            adjacencies: array2::Array2::from_size(size),
        }
    }

    pub fn size(&self) -> extents::Extents
    {
        self.states.size()
    }

    pub fn clear(&mut self)
    {
        self.states.fill_with(CellState::None);
        self.adjacencies.fill_with(0);
    }
}


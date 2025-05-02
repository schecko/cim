
use bitflags::bitflags;
use base::array2::*;
use base::extents::Extents;

use bevy::prelude::*;

bitflags!
{
    #[repr(transparent)]
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CellType: u8
    {
        const None = 0 << 0;
        const Land = 1 << 0;
    }
}

#[derive(Debug, Clone, Resource)]
pub struct TerrainGrid
{
    pub grid: Array2<CellType>,
}

impl TerrainGrid
{
    pub fn size(&self) -> Extents
    {
        self.grid.size()
    }
}

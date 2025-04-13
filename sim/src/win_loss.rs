use crate::grid::Grid;
use crate::grid::CellState;

use base::extents::Extents;
use base::extents::Point;

pub trait WinLossLogic
{
}

#[derive(Debug)]
pub struct ClassicWinLossLogic
{
}

impl WinLossLogic for ClassicWinLossLogic
{
}

impl ClassicWinLossLogic
{
    pub fn blah(&self, _grid: &mut Grid, _pos: Point) -> bool
    {
        false
    }
}

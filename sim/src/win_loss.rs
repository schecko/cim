use crate::grid::Grid;

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
    #[allow(dead_code)]
    pub fn blah(&self, _grid: &mut Grid, _pos: Point) -> bool
    {
        false
    }
}

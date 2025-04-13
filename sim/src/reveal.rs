use crate::grid::Grid;
use crate::grid::CellState;

use base::extents::Extents;
use base::extents::Point;

pub trait RevealLogic
{
    fn reveal(&self, grid: &mut Grid, pos: Point) -> Vec<Point>;
}

#[derive(Debug)]
pub struct ClassicRevealLogic
{
}

impl RevealLogic for ClassicRevealLogic
{
    fn reveal(&self, grid: &mut Grid, pos: Point) -> Vec<Point>
    {
        if let Some(cell) = grid.states.get_by_index2_mut(pos)
        {
            cell.insert(CellState::Revealed);
        }
        Vec::new()
    }
}

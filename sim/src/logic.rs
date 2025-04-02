
use crate::grid::Grid;
use crate::grid::CellState;

use base::extents::Extents;
use base::extents::Point;

#[derive(Debug)]
pub struct Logic
{
}

impl Logic
{
    pub fn guess(&self, grid: &mut Grid, pos: Point)
    {
        if let Some(cell) = grid.states.get_by_index2_mut(pos)
        {
            cell.insert(CellState::Revealed);
        }
    }
}

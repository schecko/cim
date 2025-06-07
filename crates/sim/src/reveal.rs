use crate::grid::Grid;
use crate::grid::CellState;

use base::extents::Neighbours;
use base::point::Point;

pub trait RevealLogic
{
    fn reveal(&self, grid: &mut Grid, pos: Point) -> Vec<Point>;
}

#[derive(Debug)]
pub struct ClassicRevealLogic
{
}

impl ClassicRevealLogic
{
    fn reveal_internal(&self, grid: &mut Grid, pos: Point, revealed: &mut Vec<Point>)
    {
        let cell_state = grid.states.get_by_index2_mut(pos).unwrap();
        if cell_state.intersects(CellState::Mine | CellState::Revealed | CellState::NonPlayable | CellState::Flag)
        {
            return;
        }

        cell_state.insert(CellState::Revealed);
        revealed.push(pos);

        let cell_adj = grid.adjacency.get_by_index2(pos).unwrap();
        if *cell_adj != 0
        {
            return;
        }

        for neighbour in grid.size().neighbours::<{ Neighbours::All.bits() }>(pos)
        {
            self.reveal_internal(grid, neighbour, revealed);
        }
    }
}

impl RevealLogic for ClassicRevealLogic
{
    fn reveal(&self, grid: &mut Grid, pos: Point) -> Vec<Point>
    {
        let mut revealed = Vec::new();
        self.reveal_internal(grid, pos, &mut revealed);
        revealed
    }
}

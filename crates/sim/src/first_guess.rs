use crate::grid::Grid;
use crate::logic::LogicPreview;
use crate::grid_gen;

use base::random::RandomGenerator;

pub trait FirstGuessLogic
{
    fn handle_guess(&self, grid: &mut Grid, rand: &mut RandomGenerator, preview: &LogicPreview);
}

#[derive(Debug, Default)]
pub struct SafeFirstGuessLogic
{
}

impl FirstGuessLogic for SafeFirstGuessLogic
{
    // TODO make async
    fn handle_guess(&self, grid: &mut Grid, rand: &mut RandomGenerator, preview: &LogicPreview)
    {
        grid_gen::move_mines(grid, rand, preview.pos);
        grid.first_guess = false;
    }
}

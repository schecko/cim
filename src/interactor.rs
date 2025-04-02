
use base::extents::Extents;
use base::extents::Point;
use sim::grid::Grid;
use sim::logic::Logic;
use vis::board_vis_tuning::BoardVisTuning;

use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct Interactor
{
    logic: Logic,
}

impl Interactor
{
    pub fn new() -> Self
    {
        Interactor
        {
            logic: Logic{},
        }
    }

    pub fn on_tap(&mut self, grid: &mut Grid, vis_tuning: &BoardVisTuning, world_pos: &Vec2)
    {
        let pos = (world_pos / vis_tuning.cell_size).as_ivec2();
        self.logic.guess(grid, pos);
    }
}



use sim::grid::Grid;
use sim::logic::Logic;
use sim::logic::LogicPreview;
use sim::logic::PreviewKind;
use vis::board_vis_tuning::BoardVisTuning;
use base::random::RandomGenerator;

use bevy::prelude::*;

#[derive(Resource)]
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
            logic: Logic::new(),
        }
    }

    pub fn logic(&self) -> &Logic
    {
        &self.logic
    }

    pub fn on_primary(&mut self, grid: &mut Grid, vis_tuning: &BoardVisTuning, world_pos: &Vec2)
    {
        let pos = (world_pos / vis_tuning.cell_size).as_ivec2();
        let preview = self.logic.preview_guess(grid, pos.into());
        self.actualize_preview(grid, &preview);
    }

    pub fn on_secondary(&mut self, grid: &mut Grid, vis_tuning: &BoardVisTuning, world_pos: &Vec2)
    {
        let pos = (world_pos / vis_tuning.cell_size).as_ivec2();
        let preview = self.logic.preview_flag(grid, pos.into());
        self.actualize_preview(grid, &preview);
    }

    fn actualize_preview(&mut self, grid: &mut Grid, preview: &LogicPreview)
    {
        match preview.kind
        {
            PreviewKind::FirstGuess =>
            {
                let mut rand = RandomGenerator::new(2);
                self.logic.do_first_guess(grid, &mut rand, preview);
            }
            PreviewKind::Guess =>
            {
                self.logic.do_guess(grid, preview);
            }
            PreviewKind::Flag =>
            {
                self.logic.do_flag(grid, preview);
            }
        }
    }
}


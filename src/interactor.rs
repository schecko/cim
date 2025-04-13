
use sim::grid::Grid;
use sim::logic::Logic;
use sim::logic::LogicPreview;
use sim::logic::PreviewKind;
use vis::board_vis_tuning::BoardVisTuning;

use bevy::prelude::*;

// TODO schecko
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InteractionMode
{
    Guess,
    Flag,
}

#[derive(Resource)]
pub struct Interactor
{
    mode: InteractionMode,
    logic: Logic,
}

impl Interactor
{
    pub fn new() -> Self
    {
        Interactor
        {
            mode: InteractionMode::Guess,
            logic: Logic::new(),
        }
    }

    pub fn on_tap(&mut self, grid: &mut Grid, vis_tuning: &BoardVisTuning, world_pos: &Vec2)
    {
        let pos = (world_pos / vis_tuning.cell_size).as_ivec2();

        let preview = match self.mode
        {
            InteractionMode::Guess =>
            {
                self.logic.preview_guess(grid, pos)
            },
            InteractionMode::Flag =>
            {
                self.logic.preview_flag(grid, pos)
            },
        };

        self.actualize_preview(grid, preview);
    }

    fn actualize_preview(&mut self, grid: &mut Grid, preview: LogicPreview)
    {
        match preview.kind
        {
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


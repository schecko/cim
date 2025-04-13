
use crate::grid::Grid;
use crate::grid::CellState;
use crate::reveal::RevealLogic;
use crate::reveal::ClassicRevealLogic;
use crate::win_loss::WinLossLogic;
use crate::win_loss::ClassicWinLossLogic;

use base::extents::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewKind
{
    Guess,
    Flag,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewResult
{
    Success,
    Fail,
    Nothing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicPreview
{
    pub pos: Point,
    pub kind: PreviewKind,
    pub result: PreviewResult,
}

#[derive(Debug, Clone)]
pub struct GuessResult
{
    pub pos: Point,
    pub revealed: Vec<Point>,
}

#[derive(Debug, Clone)]
pub struct FlagResult
{
    pub pos: Point,
}

pub struct Logic
{
    reveal: Box<dyn RevealLogic + Send + Sync>,
    // TODO schecko
    #[allow(dead_code)]
    win_loss: Box<dyn WinLossLogic + Send + Sync>,
}

impl Logic
{
    pub fn new() -> Self
    {
        Logic
        {
            reveal: Box::new(ClassicRevealLogic{}),
            win_loss: Box::new(ClassicWinLossLogic{}),
        }
    }

    pub fn preview_guess(&self, grid: &Grid, pos: Point) -> LogicPreview
    {
        let Some(cell) = grid.states.get_by_index2(pos) else
        {
            return LogicPreview{ pos, kind: PreviewKind::Guess, result: PreviewResult::Nothing };
        };

        if cell.intersects(CellState::Revealed | CellState::NonPlayable | CellState::Flag)
        {
            return LogicPreview{ pos, kind: PreviewKind::Guess, result: PreviewResult::Nothing };
        }

        let result = if cell.intersects(CellState::Mine)
        {
            PreviewResult::Fail
        }
        else
        {
            PreviewResult::Success
        };

        return LogicPreview{ pos, kind: PreviewKind::Guess, result };
    }

    pub fn preview_flag(&self, grid: &Grid, pos: Point) -> LogicPreview
    {
        let Some(cell) = grid.states.get_by_index2(pos) else
        {
            return LogicPreview{ pos, kind: PreviewKind::Flag, result: PreviewResult::Nothing };
        };

        if cell.intersects(CellState::Revealed | CellState::NonPlayable)
        {
            return LogicPreview{ pos, kind: PreviewKind::Flag, result: PreviewResult::Nothing };
        }

        return LogicPreview{ pos, kind: PreviewKind::Flag, result: PreviewResult::Success };
    }

    pub fn do_guess(&self, grid: &mut Grid, preview: LogicPreview) -> GuessResult
    {
        assert!(preview.kind == PreviewKind::Guess);
        assert!(self.preview_guess(grid, preview.pos) == preview);

        match preview.result
        {
            PreviewResult::Success =>
            {
                let cells = self.reveal.reveal(grid, preview.pos);
                GuessResult
                {
                    pos: preview.pos,
                    revealed: cells,
                }
            }
            PreviewResult::Fail =>
            {
                GuessResult { pos: preview.pos, revealed: Vec::new(), }
            }
            PreviewResult::Nothing =>
            {
                GuessResult { pos: preview.pos, revealed: Vec::new(), }
            }
        }
    }

    pub fn do_flag(&self, grid: &mut Grid, preview: LogicPreview) -> FlagResult
    {
        assert!(preview.kind == PreviewKind::Flag);
        assert!(self.preview_flag(grid, preview.pos) == preview);

        let cell = grid.states.get_by_index2_mut(preview.pos).unwrap();
        cell.toggle(CellState::Flag);
        FlagResult
        {
            pos: preview.pos,
        }
    }
}

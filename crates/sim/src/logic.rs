
use crate::grid::Grid;
use crate::grid::CellState;
use crate::reveal::RevealLogic;
use crate::reveal::ClassicRevealLogic;
use crate::win_loss::WinLossLogic;
use crate::win_loss::ClassicWinLossLogic;
use crate::first_guess::FirstGuessLogic;
use crate::first_guess::SafeFirstGuessLogic;

use base::random::RandomGenerator;
use base::point::Point;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum WinStatus
{
    #[default]
    InProgress,
    Win,
    Loss,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewKind
{
    FirstGuess,
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
    first_guess: Box<dyn FirstGuessLogic + Send + Sync>,
    reveal: Box<dyn RevealLogic + Send + Sync>,
    win_loss: Box<dyn WinLossLogic + Send + Sync>,
}

impl Logic
{
    pub fn new() -> Self
    {
        Logic
        {
            first_guess: Box::new(SafeFirstGuessLogic{}),
            reveal: Box::new(ClassicRevealLogic{}),
            win_loss: Box::new(ClassicWinLossLogic::default()),
        }
    }

    pub fn get_status(&self) -> WinStatus
    {
        self.win_loss.get_status()
    }

    pub fn preview_guess(&self, grid: &Grid, pos: Point) -> LogicPreview
    {
        let kind = if grid.first_guess
        {
            PreviewKind::FirstGuess
        }
        else
        {
            PreviewKind::Guess
        };
        return LogicPreview{ pos, kind, result: self.win_loss.check_guess(grid, pos) };
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

    pub fn do_first_guess
    (
        &mut self,
        grid: &mut Grid,
        rand: &mut RandomGenerator,
        original_preview: &LogicPreview
    ) -> GuessResult
    {
        assert!(original_preview.kind == PreviewKind::FirstGuess);
        assert!(self.preview_guess(grid, original_preview.pos) == *original_preview);

        if original_preview.result == PreviewResult::Nothing
        {
            return GuessResult{ pos: original_preview.pos, revealed: Vec::new() };
        }

        self.first_guess.handle_guess(grid, rand, original_preview);

        // recompute preview after the first guess logic, there may no longer be a mine there
        let preview = self.preview_guess(grid, original_preview.pos);
        self.win_loss.handle_guess(grid, &preview);
        assert!(self.win_loss.get_status() != WinStatus::Loss);
        let cells = self.reveal.reveal(grid, preview.pos);
        self.win_loss.post_reveal(grid);
        assert!(self.win_loss.get_status() != WinStatus::Loss);
        GuessResult
        {
            pos: preview.pos,
            revealed: cells,
        }
    }

    pub fn do_guess(&mut self, grid: &mut Grid, preview: &LogicPreview) -> GuessResult
    {
        assert!(preview.kind == PreviewKind::Guess);
        assert!(self.preview_guess(grid, preview.pos) == *preview);

        if preview.result == PreviewResult::Nothing
        {
            return GuessResult{ pos: preview.pos, revealed: Vec::new() };
        }

        self.win_loss.handle_guess(grid, preview);
        let cells = self.reveal.reveal(grid, preview.pos);
        self.win_loss.post_reveal(grid);
        GuessResult
        {
            pos: preview.pos,
            revealed: cells,
        }
    }

    pub fn do_flag(&self, grid: &mut Grid, preview: &LogicPreview) -> FlagResult
    {
        assert!(preview.kind == PreviewKind::Flag);
        assert!(self.preview_flag(grid, preview.pos) == *preview);

        if preview.result == PreviewResult::Nothing
        {
            return FlagResult{ pos: preview.pos };
        }

        let cell = &mut grid.states.get_by_index2_mut(preview.pos).unwrap();
        cell.toggle(CellState::Flag);
        FlagResult
        {
            pos: preview.pos,
        }
    }
}

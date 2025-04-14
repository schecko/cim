use crate::grid::Grid;
use crate::grid::CellState;
use crate::logic::PreviewResult;
use crate::logic::LogicPreview;

use base::extents::Point;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum Status
{
    #[default]
    InProgress,
    Win,
    Loss,
}

pub trait WinLossLogic
{
    fn check_guess(&self, _grid: &Grid, _pos: Point) -> PreviewResult;
    fn get_status(&self) -> Status;
    fn handle_guess(&mut self, grid: &Grid, preview: &LogicPreview);
}

#[derive(Debug, Default)]
pub struct ClassicWinLossLogic
{
    status: Status,
}

impl WinLossLogic for ClassicWinLossLogic
{
    fn check_guess(&self, grid: &Grid, pos: Point) -> PreviewResult
    {
        let Some(cell_state) = grid.states.get_by_index2(pos) else
        {
            return PreviewResult::Nothing;
        };

        if cell_state.intersects(CellState::Revealed | CellState::NonPlayable | CellState::Flag)
        {
            return PreviewResult::Nothing;
        }

        if cell_state.intersects(CellState::Mine)
        {
            PreviewResult::Fail
        }
        else
        {
            PreviewResult::Success
        }
    }

    fn handle_guess(&mut self, grid: &Grid, preview: &LogicPreview)
    {
        if self.status != Status::InProgress
        {
            return;
        }

        if preview.result == PreviewResult::Fail
        {
            self.status = Status::Loss;
            return;
        }

        if Self::is_won(grid)
        {
            self.status = Status::Win;
            return;
        }
    }

    fn get_status(&self) -> Status
    {
        self.status
    }
}

impl ClassicWinLossLogic
{
    fn is_won(grid: &Grid) -> bool
    {
        Self::validate(grid);

        for pos in grid.size().index2_space()
        {
            let cell = grid.states.get_by_index2(pos).unwrap();
            if *cell == CellState::None
            {
                return false;
            }

            if cell.contains(CellState::Flag) && !cell.contains(CellState::Mine)
            {
                return false;
            }
        }

        return true;
    }

    fn validate(grid: &Grid)
    {
        for pos in grid.size().index2_space()
        {
            let cell = grid.states.get_by_index2(pos).unwrap();
            if cell.contains(CellState::NonPlayable)
            {
                assert!(cell.bits().count_ones() == 1);
            }

            if cell.contains(CellState::Flag)
            {
                assert!(!cell.contains(CellState::Revealed));
            }
        }
    }
}

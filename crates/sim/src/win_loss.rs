use crate::grid::Grid;
use crate::grid::CellState;
use crate::logic::PreviewResult;
use crate::logic::LogicPreview;
use crate::logic::WinStatus;

use base::extents::Neighbours;
use base::point::Point;

pub trait WinLossLogic
{
    fn check_chord(&self, grid: &Grid, pos: Point) -> PreviewResult;
    fn check_guess(&self, grid: &Grid, pos: Point) -> PreviewResult;
    fn get_status(&self) -> WinStatus;
    fn handle_guess(&mut self, grid: &Grid, preview: &LogicPreview);
    fn post_reveal(&mut self, grid: &Grid);
}

#[derive(Debug, Default)]
pub struct ClassicWinLossLogic
{
    status: WinStatus,
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

    fn check_chord(&self, grid: &Grid, pos: Point) -> PreviewResult
    {
        let Some(cell_state) = grid.states.get_by_index2(pos) else
        {
            return PreviewResult::Nothing;
        };

        assert!(cell_state.contains(CellState::Revealed));
        let mut success = true;
        for neighbour in grid.size().neighbours::<{ Neighbours::All.bits() }>(pos)
        {
            let neighbour = grid.states.get_by_index2(neighbour).unwrap();
            let has_flag = neighbour.contains(CellState::Flag);
            let has_mine = neighbour.contains(CellState::Mine);
            success &= has_flag == has_mine;
        }

        if success
        {
            PreviewResult::Success
        }
        else
        {
            PreviewResult::Nothing
        }
    }

    fn handle_guess(&mut self, _grid: &Grid, preview: &LogicPreview)
    {
        if self.status != WinStatus::InProgress
        {
            return;
        }

        if preview.result == PreviewResult::Fail
        {
            self.status = WinStatus::Loss;
            return;
        }
    }
    
    fn post_reveal(&mut self, grid: &Grid)
    {
        if self.status != WinStatus::InProgress
        {
            return;
        }

        if Self::is_won(grid)
        {
            self.status = WinStatus::Win;
            return;
        }
    }

    fn get_status(&self) -> WinStatus
    {
        self.status
    }
}

impl ClassicWinLossLogic
{
    fn is_won(grid: &Grid) -> bool
    {
        Self::validate(grid);

        let mut revealed_playable = 0;
        let mut playable = 0;
        for pos in grid.size().index2_space()
        {
            let cell = grid.states.get_by_index2(pos).unwrap();

            if cell.intersects(CellState::Mine | CellState::NonPlayable)
            {
                continue;
            }
            
            playable += 1;
            if cell.contains(CellState::Revealed)
            {
                revealed_playable += 1;
            }
        }

        return revealed_playable == playable;
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

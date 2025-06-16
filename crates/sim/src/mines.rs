
use crate::grid::Grid;
use crate::grid::CellState;

use base::point::Point;
use base::random::RandomGenerator;
use base::extents::Neighbours;

pub fn initial_mines(grid: &mut Grid, rand: &mut RandomGenerator, count: u32)
{
	assert!(count > 0);

	let mut valid_locations = Vec::<u32>::new();
	valid_locations.reserve(grid.size().num_elements());
	for (i, cell) in grid.states.enumerate()
	{
		if cell.intersects(CellState::Mine | CellState::NonPlayable)
		{
			continue;
		}

		valid_locations.push(i as u32);
	}

	rand.shuffle(&mut valid_locations[..]);

	let max_mines = std::cmp::min(valid_locations.len() as u32, count);
	for i in 0..max_mines
	{
		let cell_index = valid_locations[i as usize];
		let cell = grid.states.get_by_index_mut(cell_index as usize).unwrap();
		assert!(!cell.contains(CellState::NonPlayable));
		assert!(!cell.contains(CellState::Mine));
		*cell |= CellState::Mine;
	}
	grid.update_adjacency();
}

pub fn move_mines(grid: &mut Grid, rand: &mut RandomGenerator, safe_point: Point)
{
	let mut unsafe_mines: u32 = 0;
	for neighbour in grid.size().neighbours_self::<{ Neighbours::All.bits() }>(safe_point)
	{
		if grid.states[neighbour].contains(CellState::Mine)
		{
			unsafe_mines += 1;
		}
		grid.states[neighbour].remove(CellState::Mine);
	}

	let mut valid_locations = Vec::<Point>::new();
	valid_locations.reserve(grid.size().num_elements());
	for (i, cell) in grid.states.enumerate2()
	{
		if cell.intersects(CellState::Mine | CellState::NonPlayable)
		{
			continue;
		}

		// can't use the safe cells
		if i.onion_distance(&safe_point) <= 1
		{
			continue;
		}

		valid_locations.push(i);
	}

	rand.shuffle(&mut valid_locations[..]);

	assert!(valid_locations.len() as u32 >= unsafe_mines);
	let max_mines = std::cmp::min(valid_locations.len() as u32, unsafe_mines);
	for i in 0..max_mines
	{
		let cell_index2 = valid_locations[i as usize];
		let cell = &mut grid.states[cell_index2];
		assert!(!cell.contains(CellState::NonPlayable));
		assert!(!cell.contains(CellState::Mine));
		*cell |= CellState::Mine;
	}

	grid.update_adjacency();
	
    for neighbour in grid.size().neighbours_self::<{ Neighbours::All.bits() }>(safe_point)
    {
    	assert!(!grid.states[neighbour].contains(CellState::Mine));
    }
}



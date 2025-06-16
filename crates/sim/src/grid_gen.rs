
use crate::grid::Grid;
use crate::grid::CellState;

use base::point::Point;
use base::random::RandomGenerator;
use base::extents::Neighbours;

pub fn initial_terrain(grid: &mut Grid, rand: &mut RandomGenerator, land_percent: f32)
{
	assert!(land_percent > 0.0);

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

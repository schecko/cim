
use crate::grid::Grid;
use crate::grid::CellState;

use base::random::RandomGenerator;

use noise::Perlin;
use noise::NoiseFn;

pub fn initial_terrain(grid: &mut Grid, _rand: &mut RandomGenerator, water_level: f32)
{
	assert!(water_level >= -1.0 && water_level <= 1.0);

	let perlin = Perlin::new(100);

	let size = grid.size();
	for p in grid.states.size().index2_space()
	{
		let height = perlin.get([p.x as f64 / size.width as f64, p.y as f64 / size.height as f64]);
		let is_land = height as f32 > water_level;
		
		let cell = &mut grid.states[p];
		assert!(!cell.contains(CellState::Mine));
		cell.set(CellState::NonPlayable, is_land);
	}
}

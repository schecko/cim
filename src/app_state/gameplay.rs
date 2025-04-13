
use crate::app_state::AppState;
use crate::input;
use crate::interactor::Interactor;
use crate::screens;

use base::array2::Array2;
use base::extents::Extents;
use sim::grid::*;
use vis::grid_vis::GridVis;
use vis::grid_vis;
use vis::terrain_grid::CellType;
use vis::terrain_grid::TerrainGrid;
use vis::terrain_vis;

use bevy::prelude::*;

#[derive(Component)]
pub struct GameplayAppState;

impl GameplayAppState
{
    fn on_enter(mut commands: Commands, asset_server: Res<AssetServer>)
    {
        // TODO
        let mut grid = Grid::new(5, 5);
        *grid.states.get_by_index2_mut((0, 0).into()).unwrap() = CellState::Mine;
        *grid.states.get_by_index2_mut((1, 1).into()).unwrap() = CellState::Mine;
        *grid.states.get_by_index2_mut((4, 4).into()).unwrap() = CellState::Mine;
        grid.update_adjacency();

        let size = Extents::new(5, 5);
        let mut terrain = TerrainGrid
        {
            grid: Array2::<CellType>::from_size(size),
        };
        *terrain.grid.get_by_index2_mut((0, 0).into()).unwrap() = CellType::Land;
        *terrain.grid.get_by_index2_mut((2, 2).into()).unwrap() = CellType::Land;
        *terrain.grid.get_by_index2_mut((4, 4).into()).unwrap() = CellType::Land;

        commands.insert_resource(Interactor::new());
        commands.insert_resource(GridVis{ grid });
        commands.insert_resource(terrain);

        screens::hud::spawn(commands, asset_server);
    }
}

impl Plugin for GameplayAppState
{
    fn build(&self, app: &mut App)
    {
        app
            .add_plugins(terrain_vis::TerrainVisPlugin{})
            .add_plugins(grid_vis::GridVisPlugin{})

            .add_systems(OnEnter(AppState::Gameplay), GameplayAppState::on_enter)
            .add_systems
            (
                OnEnter(AppState::Gameplay),
                (
                    grid_vis::spawn_adjacency,
                    grid_vis::spawn_covers,
                    grid_vis::spawn_grid,
                    grid_vis::spawn_mines,
                    terrain_vis::startup
                )
                    .after(GameplayAppState::on_enter)
            )
            .add_systems
            (
                Update,
                (
                     input::camera_pan,
                     input::camera_zoom,
                     input::reveal_cell,
                     grid_vis::reveal_covers
                )
                    .run_if(in_state(AppState::Gameplay))
            )
            ;
    }
}


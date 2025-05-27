
use crate::app_state::AppState;
use crate::input;
use crate::interactor::Interactor;
use crate::screens;
use crate::screens::hud::HudScreen;

use base::array2::Array2;
use base::extents::Extents;
use sim::grid::*;
use vis::grid_entities::GridVis;
use vis::grid_entities;
use vis::grid_lines;
use vis::terrain_grid::CellType;
use vis::terrain_grid::TerrainGrid;
use vis::terrain_vis;
use sim::logic::WinStatus;

use lunex::UiLayoutRoot;
use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum InitializeSet
{
    BeforeBoard,
    AfterBoard,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum UpdateSet
{
    First,
    Second,
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SubState
{
    Playing,
    End,
}

#[derive(Component)]
pub struct GameplayAppState;

impl GameplayAppState
{
    fn on_enter(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut next_state: ResMut<NextState<SubState>> )
    {
        // TODO
        let mut grid = Grid::new(5, 5);
        *grid.states.get_by_index2_mut((0, 0).into()).unwrap() = CellState::Mine;
        *grid.states.get_by_index2_mut((1, 1).into()).unwrap() = CellState::Mine;
        *grid.states.get_by_index2_mut((4, 4).into()).unwrap() = CellState::Mine;

        *grid.states.get_by_index2_mut((1, 0).into()).unwrap() = CellState::NonPlayable;
        *grid.states.get_by_index2_mut((0, 1).into()).unwrap() = CellState::NonPlayable;
        *grid.states.get_by_index2_mut((2, 2).into()).unwrap() = CellState::NonPlayable;
        *grid.states.get_by_index2_mut((4, 3).into()).unwrap() = CellState::NonPlayable;
        grid.update_adjacency();

        let size = Extents::new(5, 5);
        let mut terrain = TerrainGrid
        {
            grid: Array2::<CellType>::from_size(size),
        };
        *terrain.grid.get_by_index2_mut((1, 0).into()).unwrap() = CellType::Land;
        *terrain.grid.get_by_index2_mut((0, 1).into()).unwrap() = CellType::Land;
        *terrain.grid.get_by_index2_mut((2, 2).into()).unwrap() = CellType::Land;
        *terrain.grid.get_by_index2_mut((4, 3).into()).unwrap() = CellType::Land;

        commands.insert_resource(Interactor::new());
        commands.insert_resource(GridVis{ grid });
        commands.insert_resource(terrain);
        next_state.set(SubState::Playing);

        screens::hud::spawn(commands, asset_server);
    }

    fn on_update
    (
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        interactor: Res<Interactor>,
        mut next_state: ResMut<NextState<SubState>>,
        hud_screen: Option<Single<Entity, (With<HudScreen>, With<UiLayoutRoot>)>>,
    )
    {
        let status =  interactor.logic().get_status();
        if status != WinStatus::InProgress
        {
            if let Some(hud) = hud_screen
            {
                commands.entity(*hud).despawn();
            }
            screens::eog::spawn(commands, asset_server);
            next_state.set(SubState::End);
        }
    }
}

impl Plugin for GameplayAppState
{
    fn build(&self, app: &mut App)
    {
        app
            .add_plugins(terrain_vis::TerrainVisPlugin{})
            .add_plugins(grid_entities::GridEntitiesPlugin{})
            .add_plugins(grid_lines::GridLinesPlugin{})
            
            .insert_state(SubState::Playing)

            // initialize
            .add_systems
            (
                OnEnter(AppState::Gameplay),
                (
                    GameplayAppState::on_enter,
                    grid_entities::init_handles
                )
                .in_set(InitializeSet::BeforeBoard)
            )
            .add_systems
            (
                OnEnter(AppState::Gameplay),
                (
                    grid_entities::init_known,
                    grid_entities::spawn_adjacency,
                    grid_lines::spawn_lines,
                    terrain_vis::startup
                )
                .in_set(InitializeSet::AfterBoard)
            )

            .add_systems
            (
                OnExit(AppState::Gameplay),
                (
                    grid_entities::destroy_known,
                    grid_lines::despawn_lines,
                    terrain_vis::shutdown
                )
            )

            // update while playing
            .add_systems
            (
                Update,
                (
                    GameplayAppState::on_update,
                    input::camera_pan,
                    input::camera_zoom,
                    input::reveal_cell,
                    grid_entities::sync_grid_entities::<grid_entities::Mine>,
                    grid_entities::sync_grid_entities::<grid_entities::Flag>,
                    grid_entities::sync_grid_entities::<grid_entities::Cover>,
                )
                .run_if(in_state(AppState::Gameplay))
                .run_if(in_state(SubState::Playing))
            )

            // update in eog
            .add_systems
            (
                Update,
                (
                    input::camera_pan,
                    input::camera_zoom,
                )
                .run_if(in_state(AppState::Gameplay))
                .run_if(in_state(SubState::End))
            )

            // sets
            .configure_sets
            (
                OnEnter(AppState::Gameplay),
                (
                    InitializeSet::AfterBoard
                        .after(InitializeSet::BeforeBoard),
                )
            )
            .configure_sets
            (
                Update,
                (
                    UpdateSet::First
                        .run_if(in_state(AppState::Gameplay)),
                    UpdateSet::Second
                        .run_if(in_state(AppState::Gameplay)),
                )
            )
            ;
    }
}


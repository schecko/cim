
use crate::app_state::AppState;
use crate::input;
use crate::interactor::Interactor;
use crate::screens;
use crate::screens::hud::HudScreen;

use base::random::RandomGenerator;
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
use sim::mines;
use sim::grid_gen;

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

#[derive(Resource, Debug, Clone)]
pub struct GameConfig
{
    pub guessing: bool,
    pub width: u32,
    pub height: u32,
    pub mine_count: u32,
}

impl GameConfig
{
    pub fn sanitize(&mut self)
    {
        self.width = self.width.clamp(1, 10000);
        self.height = self.height.clamp(1, 10000);
        self.mine_count = self.mine_count.clamp(1, self.width * self.height - 9);
    }
}

impl Default for GameConfig
{
    fn default() -> Self
    {
        Self
        {
            guessing: false,
            width: 20,
            height: 20,
            mine_count: 20 * 20 / 3,
        }
    }
}

#[derive(Component)]
pub struct GameplayAppState;

impl GameplayAppState
{
    fn on_enter(
        mut commands: Commands,
        config: Res<GameConfig>,
        asset_server: Res<AssetServer>,
        mut next_state: ResMut<NextState<SubState>> )
    {
        let mut grid = Grid::new(config.width as i32, config.height as i32);
        let mut rand = RandomGenerator::new(1);

        grid_gen::initial_terrain(&mut grid, &mut rand, 0.5);
        mines::initial_mines(&mut grid, &mut rand, config.mine_count);

        let mut terrain = TerrainGrid
        {
            grid: Array2::<CellType>::from_size(grid.size()),
        };
        for i in grid.size().index_space()
        {
            terrain.grid[i] = if grid.states[i].contains(CellState::NonPlayable)
            {
                CellType::Land
            }
            else
            {
                CellType::None
            };
        }

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
            println!("game finished, status: {:?}", status);
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
                    grid_entities::sync_adjacency,
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


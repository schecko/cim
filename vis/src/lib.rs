
pub mod board_vis_tuning;
pub mod grid_vis;
pub mod terrain_grid;
mod terrain_vis;
mod layers;

use base::tuning::Tuning;
use bevyx::ron::RonAssetPlugin;
use board_vis_tuning::*;

use bevy::prelude::*;

pub fn hello_vis()
{
    println!("Hello, vis!");
}

fn pre_startup
(
    mut commands: Commands,
)
{
    BoardVisTuning::default().save();
    commands.insert_resource(BoardVisTuning::load());
}

pub struct GameVisPlugin;

impl Plugin for GameVisPlugin
{
    fn build(&self, app: &mut App)
    {
        app
            .add_plugins(RonAssetPlugin::<BoardVisTuning>::default())
            .add_plugins(terrain_vis::TerrainVisPlugin{})
            .add_plugins(grid_vis::GridVisPlugin{})
            .add_systems(PreStartup, pre_startup);
    }
}

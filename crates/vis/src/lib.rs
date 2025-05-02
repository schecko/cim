
pub mod board_vis_tuning;
pub mod grid_entities;
pub mod grid_lines;
pub mod terrain_grid;
pub mod terrain_vis;
mod layers;

use board_vis_tuning::*;

use base::tuning::Tuning;
use bevyx::ron::RonAssetPlugin;

use bevy::prelude::*;

pub fn hello_vis()
{
    println!("Hello, vis!");
}

fn startup
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
            .add_systems(Startup, startup);
    }
}

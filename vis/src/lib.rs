
mod board_vis_tuning;
mod grid_vis;
mod terrain_vis;

use base::array2::*;
use base::extents::*;
use base::ronx::*;
use base::tuning::Tuning;
use bevyx::ron::RonAssetPlugin;
use board_vis_tuning::*;

use bevy::math::VectorSpace;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_asset::*;
use bevy::render::render_resource::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;
use bevy::sprite::*;
use bitflags::bitflags;

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

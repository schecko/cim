
use base::array2::*;
use base::extents::*;
use base::ronx::*;
use base::tuning::Tuning;
use bevyx::ron::RonAssetPlugin;
use crate::board_vis_tuning::*;

use bevy::math::VectorSpace;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_asset::*;
use bevy::render::render_resource::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;
use bevy::sprite::*;
use bitflags::bitflags;

pub struct GridVisPlugin;

impl Plugin for GridVisPlugin
{
    fn build(&self, _app: &mut App)
    {
        /*app
            .add_plugins(Material2dPlugin::<TerrainMaterial>::default())
            .add_systems(Startup, startup);
            */
    }
}

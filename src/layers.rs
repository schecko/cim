

use bevy::render::view::visibility::RenderLayers;

pub const GAME_LAYER: usize = 0;
pub const UI_LAYER: usize = 1;
pub const DEBUG_LAYER_2D: usize = 2;
pub const DEBUG_LAYER_3D: usize = 3;

pub const UI_RENDER_LAYER: RenderLayers = RenderLayers::layer(UI_LAYER);

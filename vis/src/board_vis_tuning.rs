
use bevy::prelude::*;
use bevy::reflect::TypePath;

#[derive(serde::Deserialize, serde::Serialize, Asset, TypePath, Default)]
pub struct BoardVisTuning
{
    cell_size: Vec2,
}


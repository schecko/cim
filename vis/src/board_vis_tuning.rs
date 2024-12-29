
use base::tuning::Tuning;
use bevy::prelude::*;
use bevy::reflect::TypePath;

#[derive(serde::Deserialize, serde::Serialize, Asset, Resource, TypePath)]
pub struct BoardVisTuning 
{
    pub cell_size: Vec2,
}

impl Tuning for BoardVisTuning
{
    fn path() -> &'static std::path::Path
    {
        &std::path::Path::new("tuning/board_vis.ron")
    }
}

impl Default for BoardVisTuning
{
    fn default() -> Self
    {
        Self
        {
            cell_size: Vec2::splat(28.0),
        }
    }
}

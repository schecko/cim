
use base::tuning::Tuning;
use bevy::prelude::*;
use bevy::reflect::TypePath;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Grid 
{
    pub world_line_width: f32, // world space
    pub uv_width: f32, // uv/normalized (0.0, 1.0)
}

impl Default for Grid
{
    fn default() -> Self
    {
        Self
        {
            world_line_width: 1.0,
            uv_width: 0.5,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Asset, Resource, TypePath)]
pub struct BoardVisTuning 
{
    pub cell_size: Vec2,
    pub grid: Grid,
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
            grid: Default::default(),
        }
    }
}

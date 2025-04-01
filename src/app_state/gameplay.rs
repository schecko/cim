
use bevy::prelude::*;
use crate::screens;
use crate::input;
use crate::app_state::AppState;

#[derive(Component)]
pub struct GameplayAppState;

impl GameplayAppState
{
    fn spawn(commands: Commands, asset_server: Res<AssetServer>)
    {
        screens::hud::spawn(commands, asset_server);
    }
}

impl Plugin for GameplayAppState
{
    fn build(&self, app: &mut App)
    {
        app
            .add_systems(OnEnter(AppState::Gameplay), GameplayAppState::spawn)
            .add_systems
            (
                Update,
                 (input::camera_pan, input::camera_zoom, input::reveal_cell)
                    .run_if(in_state(AppState::Gameplay))
            )
            // .add_systems(OnExit(AppState::Gameplay), despawn_scene::<GameplayAppState>)
            ;
    }
}


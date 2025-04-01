
use bevy::prelude::*;
use crate::screens;
use crate::app_state::AppState;

#[derive(Component)]
pub struct FrontendAppState;

impl FrontendAppState
{
    fn spawn(commands: Commands, asset_server: Res<AssetServer>)
    {
        screens::home::spawn(commands, asset_server);
    }
}

impl Plugin for FrontendAppState
{
    fn build(&self, app: &mut App)
    {
        app
            .add_systems(OnEnter(AppState::Frontend), FrontendAppState::spawn)
            // .add_systems(OnExit(AppState::Frontend), despawn_scene::<FrontendAppState>)
            // .add_systems(OnExit(AppState::Splash), despawn_scene::<SplashAppState>)
            ;
    }
}


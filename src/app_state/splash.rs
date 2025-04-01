
use bevy::prelude::*;
use crate::screens;
use crate::app_state::AppState;

#[derive(Component)]
pub struct SplashAppState;

impl SplashAppState
{
    fn spawn(commands: Commands, asset_server: Res<AssetServer>)
    {
        screens::splash::spawn(commands, asset_server);
    }
}

impl Plugin for SplashAppState
{
    fn build(&self, app: &mut App)
    {
        app
            .add_systems(OnEnter(AppState::Splash), SplashAppState::spawn)
            // .add_systems(OnExit(AppState::Splash), despawn_scene::<SplashAppState>)
            ;
    }
}

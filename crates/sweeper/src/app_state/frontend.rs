
use crate::screens;
use crate::app_state::AppState;
use crate::app_state::gameplay::GameConfig;

use bevy::prelude::*;

#[derive(Component)]
pub struct FrontendAppState;

#[derive(Event)]
pub struct ScreenUpdate
{
}

impl FrontendAppState
{
    fn spawn(commands: Commands, asset_server: Res<AssetServer>)
    {
        screens::home::spawn(commands, asset_server);
    }

    fn update(world: &mut World)
    {
        // TODO move out of frontend state, can be used anytime by a screen
        world.trigger(ScreenUpdate{});
    }
}

impl Plugin for FrontendAppState
{
    fn build(&self, app: &mut App)
    {
        app
            .add_systems(OnEnter(AppState::Frontend), FrontendAppState::spawn)
            .add_systems(Update, FrontendAppState::update)
            .insert_resource(GameConfig::default())
            ;
    }
}


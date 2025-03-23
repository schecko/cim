
use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState
{
    Splash,
    Frontend,
    Gameplay,
}

pub mod frontend;
pub mod gameplay;
pub mod splash;

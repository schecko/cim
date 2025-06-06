
use bevy::prelude::*;

pub mod frontend;
pub mod gameplay;
pub mod splash;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState
{
    Splash,
    Frontend,
    Gameplay,
}

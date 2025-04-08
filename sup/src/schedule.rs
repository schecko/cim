use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum SetupSet
{
    PreLoad,
    Load,
    PostLoad,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum UpdateSet
{
    First,
    PreUpdate,
    Update,
    PostUpdate,
    Last,
}



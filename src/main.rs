use bevy::prelude::*;

fn hello_world() {
    println!("hello world!");
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(Update, hello_world);
        println!("hello world2!");
    }
}

fn main()
{
    App::new()
        .add_plugins(HelloPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}

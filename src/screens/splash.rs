use bevy::prelude::*;
use bevy_lunex::*;

use crate::layers;
use crate::app_state::AppState;

#[derive(Component)]
struct SplashScreen;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>)
{
    commands.spawn((
        UiLayoutRoot::new_2d(),
        UiFetchFromCamera::<{ layers::UI_LAYER }>,
        layers::UI_RENDER_LAYER,
        SplashScreen,
        Name::new("Splash"),
    )).with_children(|ui|
    {
        ui.spawn
        ((
            Name::new("Background"),
            UiLayout::solid().pack(),
            UiColor::from(Color::srgb(1.0, 0.0, 1.0)),
            Sprite::from_image(asset_server.load("textures/sample.png")),
            layers::UI_RENDER_LAYER,
            SplashScreen,
        ))
        .observe(
        |
             _: Trigger<Pointer<Click>>,
             mut next: ResMut<NextState<AppState>>,
             query: Query<Entity, With<SplashScreen>>,
             mut cmd: Commands,
        |
        {
            for entity in &query
            {
                cmd.entity(entity).despawn_recursive();
            }
            next.set(AppState::Frontend);
        });
    });
}

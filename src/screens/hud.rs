use bevy::prelude::*;
use lunex::prelude::*;

use crate::layers;
use crate::app_state::AppState;

#[derive(Component)]
struct HudScreen;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>)
{
    commands.spawn((
        UiLayoutRoot::new_2d(),
        UiFetchFromCamera::<{ layers::UI_LAYER }>,
        layers::UI_RENDER_LAYER,
        HudScreen,
        Name::new("Gameplay"),
    )).with_children(|ui|
    {
        ui.spawn
        ((
            Name::new("Background"),
            UiLayout::window().anchor(Anchor::TopLeft).size((100.0, 100.0)).pack(),
            UiColor::from(Color::srgb(1.0, 1.0, 0.0)),
            Sprite::from_image(asset_server.load("textures/sample.png")),
            layers::UI_RENDER_LAYER,
            HudScreen,
        ))
        .observe(
        |
             _: Trigger<Pointer<Click>>,
             mut next: ResMut<NextState<AppState>>,
             query: Query<Entity, With<HudScreen>>,
             mut cmd: Commands,
        |
        {
            for entity in &query
            {
                cmd.entity(entity).despawn();
            }
            next.set(AppState::Frontend);
        });
    });
}

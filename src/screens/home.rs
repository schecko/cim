use bevy::prelude::*;
use bevy_lunex::*;

use crate::layers;
use crate::app_state::AppState;

#[derive(Component)]
struct HomeScreen;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>)
{
    commands.spawn((
        UiLayoutRoot::new_2d(),
        UiFetchFromCamera::<{ layers::UI_LAYER }>,
        layers::UI_RENDER_LAYER,
        HomeScreen,
        Name::new("Frontend"),
    )).with_children(|ui|
    {
        ui.spawn
        ((
            Name::new("Background"),
            UiLayout::solid().pack(),
            UiColor::from(Color::srgb(0.0, 1.0, 1.0)),
            Sprite::from_image(asset_server.load("textures/sample.png")),
            layers::UI_RENDER_LAYER,
            HomeScreen,
        ));

        ui.spawn
        ((
            Name::new("ButtonContainer"),
            UiLayout::solid().pack(),
            layers::UI_RENDER_LAYER,
            HomeScreen,
        ))
        .with_children(|ui|
        {
            let gap = 3.0;
            let size = 14.0;
            let mut offset = 0.0;
            for button in ["Play", "Settings", "Credits", "Quit Game"]
            {
                ui.spawn((
                    Name::new(button),
                    UiLayout::window().y(Rl(offset)).size(Rl((100.0, size))).pack(),
                    Text2d::new(button),
                    layers::UI_RENDER_LAYER,
                    HomeScreen,
                ))
                .observe(
                |
                     _: Trigger<Pointer<Click>>,
                     mut next: ResMut<NextState<AppState>>,
                     query: Query<Entity, With<HomeScreen>>,
                     mut cmd: Commands,
                |
                {
                    for entity in &query
                    {
                        cmd.entity(entity).despawn_recursive();
                    }
                    next.set(AppState::Gameplay);
                });
                offset += gap + size;
            }
        });
    });
}

use crate::app_state::AppState;
use crate::layers;

use bevy::prelude::*;
use lunex::*;
use strum::EnumIter;

#[derive(Component)]
struct HomeScreen;

#[derive(Debug, Eq, PartialEq, EnumIter, strum::Display)]
enum Buttons
{
    Play,
    Settings,
    Credits,
    Quit,
}

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
            let mut make_button = |button_type: Buttons|
            {
                let local_offset = offset;
                offset += gap + size;
                (
                    Name::new(button_type.to_string()),
                    UiLayout::window().y(Rl(local_offset)).size(Rl((25.0, size))).pack(),
                    layers::UI_RENDER_LAYER,
                    HomeScreen,
                )
            };

            let make_button_child = |button_type: Buttons|
            {
                (
                    Name::new("Button Text"),
                    UiColor::new(vec![
                        (UiBase::id(), Color::srgba(1.0, 0.0, 0.0, 1.0)),
                        (UiHover::id(), Color::srgba(1.0, 0.0, 1.0, 1.0))
                    ]),
                    Text2d::new(button_type.to_string()),
                    layers::UI_RENDER_LAYER,
                    HomeScreen,
                    Pickable::IGNORE,
                )
            };

            ui.spawn(make_button(Buttons::Play))
                .with_children(|ui|
                {
                    ui.spawn(make_button_child(Buttons::Play));
                })
                .observe(
                |
                     _: Trigger<Pointer<Click>>,
                     mut next: ResMut<NextState<AppState>>,
                     query: Query<Entity, With<HomeScreen>>,
                     mut cmd: Commands,
                |
                {
                    println!("play");
                    for entity in &query
                    {
                        cmd.entity(entity).despawn();
                    }
                    next.set(AppState::Gameplay);
                });

            ui.spawn(make_button(Buttons::Settings))
                .with_children(|ui|
                {
                    ui.spawn(make_button_child(Buttons::Settings));
                })
                .observe(
                |
                     _: Trigger<Pointer<Click>>,
                |
                {
                    println!("settings");
                });

            ui.spawn(make_button(Buttons::Credits))
                .with_children(|ui|
                {
                    ui.spawn(make_button_child(Buttons::Credits));
                })
                .observe(
                |
                     _: Trigger<Pointer<Click>>,
                |
                {
                    println!("credits");
                });

            ui.spawn(make_button(Buttons::Quit))
                .with_children(|ui|
                {
                    ui.spawn(make_button_child(Buttons::Quit));
                })
                .observe(
                |
                     _: Trigger<Pointer<Click>>,
                    mut exit: EventWriter<AppExit>,
                |
                {
                    exit.write(AppExit::Success);
                });
        });
    });
}

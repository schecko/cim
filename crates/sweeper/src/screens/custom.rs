use crate::app_state::AppState;
use crate::layers;
use crate::screens;
use crate::app_state::gameplay::GameConfig;
use crate::app_state::frontend::ScreenUpdate;

use bevy::prelude::*;
use lunex::*;
use strum::EnumIter;

#[derive(Component)]
struct CustomScreen;

#[derive(Debug, Eq, PartialEq, EnumIter, strum::Display)]
enum Buttons
{
    WidthInc,
    WidthDec,
    HeightInc,
    HeightDec,
    MinesInc,
    MinesDec,
    Play,
    Return,
}

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>)
{
    commands.spawn((
        UiLayoutRoot::new_2d(),
        UiFetchFromCamera::<{ layers::UI_LAYER }>,
        layers::UI_RENDER_LAYER,
        CustomScreen,
        Name::new("Custom"),
    )).with_children(|ui|
    {
        ui.spawn
        ((
            Name::new("Background"),
            UiLayout::solid().pack(),
            UiColor::from(Color::srgb(0.0, 1.0, 1.0)),
            Sprite::from_image(asset_server.load("textures/sample.png")),
            layers::UI_RENDER_LAYER,
            CustomScreen,
        ));

        ui.spawn
        ((
            Name::new("ButtonContainer"),
            UiLayout::solid().pack(),
            layers::UI_RENDER_LAYER,
            CustomScreen,
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
                    CustomScreen,
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
                    CustomScreen,
                    Pickable::IGNORE,
                )
            };

            ui.spawn(make_button(Buttons::WidthInc))
                .with_children(|ui|
                {
                    ui.spawn(make_button_child(Buttons::WidthInc));
                })
                .observe(
                |
                     _: Trigger<Pointer<Click>>,
                     mut config: ResMut<GameConfig>,
                |
                {
                    config.width += 1;
                    config.sanitize();
                });

            ui.spawn(make_button(Buttons::WidthInc))
                .with_children(|ui|
                {
                    ui.spawn(make_button_child(Buttons::WidthInc));
                })
                .observe(
                |
                     _: Trigger<Pointer<Click>>,
                     mut config: ResMut<GameConfig>,
                |
                {
                    config.width += 1;
                    config.sanitize();
                })
                .observe(
                |
                     trigger: Trigger<ScreenUpdate>,
                     config: Res<GameConfig>,
                     mut txts: Query<&mut Text2d>,
                |
                {
                    if let Ok(mut txt) = txts.get_mut(trigger.target())
                    {
                        txt.0 = config.width.to_string();
                    }
                });

            ui.spawn(make_button(Buttons::Play))
                .with_children(|ui|
                {
                    ui.spawn(make_button_child(Buttons::Play));
                })
                .observe(
                |
                     _: Trigger<Pointer<Click>>,
                     mut next: ResMut<NextState<AppState>>,
                     screen: Option<Single<Entity, (With<CustomScreen>, With<UiLayoutRoot>)>>,
                     mut cmd: Commands,
                |
                {
                    println!("play");
                    if let Some(entity) = screen
					{
                    	cmd.entity(*entity).despawn();
					}
                    next.set(AppState::Gameplay);
                });

            ui.spawn(make_button(Buttons::Return))
                .with_children(|ui|
                {
                    ui.spawn(make_button_child(Buttons::Return));
                })
                .observe(
                |
                    _: Trigger<Pointer<Click>>,
                    screen: Option<Single<Entity, (With<CustomScreen>, With<UiLayoutRoot>)>>,
                    mut cmd: Commands,
                    a_serv: Res<AssetServer>,
                |
                {
                    if let Some(entity) = screen
					{
                    	cmd.entity(*entity).despawn();
					}
                    screens::home::spawn(cmd, a_serv);
                });
        });
    });
}

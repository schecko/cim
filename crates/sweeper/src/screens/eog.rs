 use crate::app_state::AppState;
use crate::layers;

use bevy::prelude::*;
use lunex::*;
use strum::EnumIter;

#[derive(Component)]
struct EndGameScreen;

#[derive(Debug, Eq, PartialEq, EnumIter, strum::Display)]
enum Buttons
{
    Return,
}

pub fn spawn(mut commands: Commands, _asset_server: Res<AssetServer>)
{
    commands.spawn((
        UiLayoutRoot::new_2d(),
        UiFetchFromCamera::<{ layers::UI_LAYER }>,
        layers::UI_RENDER_LAYER,
        EndGameScreen,
        Name::new("EndGameScreen"),
    )).with_children(|ui|
    {
        ui.spawn
        ((
            Name::new("ButtonContainer"),
            UiLayout::solid().pack(),
            layers::UI_RENDER_LAYER,
            EndGameScreen,
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
                    EndGameScreen,
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
                    EndGameScreen,
                    Pickable::IGNORE,
                )
            };

            ui.spawn(make_button(Buttons::Return))
                .with_children(|ui|
                {
                    ui.spawn(make_button_child(Buttons::Return));
                })
                .observe(
                |
                     _: Trigger<Pointer<Click>>,
                     mut next: ResMut<NextState<AppState>>,
                     screen: Option<Single<Entity, (With<EndGameScreen>, With<UiLayoutRoot>)>>,
                     mut cmd: Commands,
                |
                {
                    if let Some(entity) = screen
					{
                    	cmd.entity(*entity).despawn();
					}
                    next.set(AppState::Frontend);
                });
        });
    });
}

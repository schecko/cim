
use crate::app_state::AppState;
use crate::layers;
use crate::screens;
use crate::app_state::gameplay::GameConfig;

use bevy::prelude::*;
use strum::EnumIter;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct CustomScreen;

#[derive(Component, Debug, Eq, PartialEq, EnumIter, strum::Display)]
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

fn return_button(txt: &str, asset_server: &AssetServer) -> impl Bundle + use<>
{
    (
        Node
        {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        layers::UI_RENDER_LAYER,
        CustomScreen,
        children!
        [(
            Button,
            Node
            {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            layers::UI_RENDER_LAYER,
            children!
            [(,
                Text::new(""),
                TextFont
                {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow::default(),
                layers::UI_RENDER_LAYER,
            )]
        )],
    )
}

fn return_button_system
(
    mut interaction_query: Query
    <
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query
    {
        match *interaction
        {
            Interaction::Pressed =>
            {
                *color = PRESSED_BUTTON.into();
                border_color.0 = bevy::color::palettes::basic::RED.into();
            }
            Interaction::Hovered =>
            {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None =>
            {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

impl CustomScreen
{
    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>)
    {
        commands.spawn
        ((
            Node
            {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
            
                ..default()
            },
            layers::UI_RENDER_LAYER,
        ))
        .with_children(|builder|
        {
            builder.spawn
            ((
                Node
                {
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                layers::UI_RENDER_LAYER,
            ))
            .with_children(|builder|
            {
                builder.spawn(return_button(Buttons::WidthInc, &asset_server))
                    .observe(
                    |
                         _: Trigger<Pointer<Click>>,
                         mut config: ResMut<GameConfig>,
                    |
                    {
                        config.width += 1;
                        config.sanitize();
                    });

                builder.spawn(return_button(Buttons::WidthDec, &asset_server))
                    .observe(
                    |
                         _: Trigger<Pointer<Click>>,
                         mut config: ResMut<GameConfig>,
                    |
                    {
                        config.width -= 1;
                        config.sanitize();
                    });
            })
            ;
        })
        ;
    }
}

impl Plugin for CustomScreen
{
    fn build(&self, app: &mut App)
    {
        app
            .add_systems(Update, return_button_system)
        ;
    }
}

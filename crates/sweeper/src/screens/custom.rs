use crate::app_state::AppState;
use crate::layers;
use crate::screens;
use crate::app_state::gameplay::GameConfig;
use crate::app_state::frontend::ScreenUpdate;

use bevy::prelude::*;
use strum::EnumIter;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct CustomScreen;

#[derive(Component, Debug, Eq, PartialEq, EnumIter, strum::Display)]
enum DynamicText
{
    Width,
    Height,
    Mines
}

fn basic_button(txt: &str, _asset_server: &AssetServer) -> impl Bundle + use<>
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
        children!
        [(
            Button,
            Node
            {
                width: Val::Px(65.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            layers::UI_RENDER_LAYER,
            children!
            [(
                Text::new(txt),
                TextFont
                {
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

fn button_system
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

fn dynamic_text_system
(
    mut text_query: Query<(&mut Text, &DynamicText)>,
    config: Res<GameConfig>,
) {
    for (mut text, ty) in &mut text_query
    {
        match ty
        {
            DynamicText::Width =>
            {
                // TODO: Local
                // text.0 = format!("width {}", config.width);
                text.0 = config.width.to_string()
            }
            DynamicText::Height =>
            {
                // TODO: Local
                text.0 = config.height.to_string();
            }
            DynamicText::Mines =>
            {
                // TODO: Local
                text.0 = config.mine_count.to_string();
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
            CustomScreen,
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
                builder.spawn
                ((
                    Text::new("width"),
                    TextFont
                    {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    TextShadow::default(),
                    layers::UI_RENDER_LAYER,
                ));

                builder.spawn(basic_button("+", &asset_server))
                    .observe(
                    |
                         _: Trigger<Pointer<Click>>,
                         mut config: ResMut<GameConfig>,
                    |
                    {
                        config.width += 1;
                        config.sanitize();
                    });

                builder.spawn
                ((
                    DynamicText::Width,
                    Text::default(),
                    TextFont
                    {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    TextShadow::default(),
                    layers::UI_RENDER_LAYER,
                ));

                builder.spawn(basic_button("-", &asset_server))
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
                builder.spawn
                ((
                    Text::new("height"),
                    TextFont
                    {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    TextShadow::default(),
                    layers::UI_RENDER_LAYER,
                ));

                builder.spawn(basic_button("+", &asset_server))
                    .observe(
                    |
                         _: Trigger<Pointer<Click>>,
                         mut config: ResMut<GameConfig>,
                    |
                    {
                        config.height += 1;
                        config.sanitize();
                    });

                builder.spawn
                ((
                    DynamicText::Height,
                    Text::default(),
                    TextFont
                    {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    TextShadow::default(),
                    layers::UI_RENDER_LAYER,
                ));

                builder.spawn(basic_button("-", &asset_server))
                    .observe(
                    |
                         _: Trigger<Pointer<Click>>,
                         mut config: ResMut<GameConfig>,
                    |
                    {
                        config.height -= 1;
                        config.sanitize();
                    });
            })
            ;
            
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
                builder.spawn
                ((
                    Text::new("mine_count"),
                    TextFont
                    {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    TextShadow::default(),
                    layers::UI_RENDER_LAYER,
                ));

                builder.spawn(basic_button("+", &asset_server))
                    .observe(
                    |
                         _: Trigger<Pointer<Click>>,
                         mut config: ResMut<GameConfig>,
                    |
                    {
                        config.mine_count += 1;
                        config.sanitize();
                    });

                builder.spawn
                ((
                    DynamicText::Mines,
                    Text::new(""),
                    TextFont
                    {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    TextShadow::default(),
                    layers::UI_RENDER_LAYER,
                ));

                builder.spawn(basic_button("-", &asset_server))
                    .observe(
                    |
                         _: Trigger<Pointer<Click>>,
                         mut config: ResMut<GameConfig>,
                    |
                    {
                        config.mine_count -= 1;
                        config.sanitize();
                    });
            })
            ;


            builder.spawn(basic_button("play", &asset_server))
                .observe(
                |
                     _: Trigger<Pointer<Click>>,
                     mut next: ResMut<NextState<AppState>>,
                     screen: Option<Single<Entity, With<CustomScreen>>>,
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

            builder.spawn(basic_button("return", &asset_server))
                .observe(
                |
                    _: Trigger<Pointer<Click>>,
                    screen: Option<Single<Entity, With<CustomScreen>>>,
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

        })
        ;
    }
}

impl Plugin for CustomScreen
{
    fn build(&self, app: &mut App)
    {
        app
            .add_systems(Update, button_system)
            .add_systems(Update, dynamic_text_system)
        ;
    }
}

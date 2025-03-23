
mod debug;
mod input;
mod layers;
use crate::input::GameplayCamera;

use bevy::dev_tools::fps_overlay::FpsOverlayConfig;
use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy::prelude::*;
use bevy::render::view::visibility::RenderLayers;
use bevy::render::RenderPlugin;
use bevy::render::settings::Backends;
use bevy::render::settings::WgpuSettings;
use bevy_egui::EguiPlugin;
use bevy_lunex::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState
{
    Splash,
    Frontend,
    Gameplay,
}

fn find_assets_folder() -> Result<(), std::io::Error>
{
    let mut current_dir = std::env::current_dir()?;

    while !current_dir.as_os_str().is_empty()
    {
        let assets_path = current_dir.join(base::assets::ASSETS_FOLDER);
        if assets_path.is_dir()
        {
            std::env::set_current_dir(&current_dir)?;
            std::env::set_var(bevyx::helper::ASSET_ROOT_ENV, &current_dir);
            return Ok(());
        }
        current_dir = match current_dir.parent()
        {
            Some(inner) => inner.to_path_buf(),
            _ => break,
        };
    }

    Err(std::io::Error::new(std::io::ErrorKind::Other, "Could not find assets folder"))
}

fn setup
(
    mut commands: Commands,
)
{
    commands.spawn
    ((
        Camera2d::default(),
        Camera
        {
            order: layers::GAME_LAYER as isize,
            ..default()
        },
        GameplayCamera,
        UiSourceCamera::<{ layers::GAME_LAYER }>,
    ));
    commands.spawn
    ((
        Camera2d::default(),
        Camera
        {
            order: layers::UI_LAYER as isize,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        UiSourceCamera::<{ layers::UI_LAYER }>,
        RenderLayers::from_layers(&[layers::UI_LAYER, layers::DEBUG_LAYER_2D]),
        Transform::from_translation(Vec3::Z * 1000.0),
    ));
}

fn despawn_scene<S: Component>(mut commands: Commands, query: Query<Entity, With<S>>)
{
    for entity in &query
    {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
struct SplashAppState;
impl SplashAppState
{
    fn spawn( mut commands: Commands, asset_server: Res<AssetServer> )
    {
        commands.spawn((
            UiLayoutRoot::new_2d(),
            UiFetchFromCamera::<{ layers::UI_LAYER }>,
            layers::UI_RENDER_LAYER,
            SplashAppState,
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
            ))
            .observe(|_: Trigger<Pointer<Click>>, mut next: ResMut<NextState<AppState>>|
            {
                next.set(AppState::Frontend);
            });
        });
    }
}

#[derive(Component)]
struct FrontendAppState;
impl FrontendAppState
{
    fn spawn( mut commands: Commands, asset_server: Res<AssetServer> )
    {
        commands.spawn((
            UiLayoutRoot::new_2d(),
            UiFetchFromCamera::<{ layers::UI_LAYER }>,
            layers::UI_RENDER_LAYER,
            FrontendAppState,
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
            ));

            ui.spawn
            ((
                Name::new("ButtonContainer"),
                UiLayout::solid().pack(),
                layers::UI_RENDER_LAYER,
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
                        ))
                        .observe(|_: Trigger<Pointer<Click>>, mut next: ResMut<NextState<AppState>>|
                        {
                            next.set(AppState::Gameplay);
                        });
                        offset += gap + size;
                    }
            });


        });
    }
}

#[derive(Component)]
struct GameplayAppState;
impl GameplayAppState
{
    fn spawn( mut commands: Commands, asset_server: Res<AssetServer> )
    {
        commands.spawn((
            UiLayoutRoot::new_2d(),
            UiFetchFromCamera::<{ layers::UI_LAYER }>,
            layers::UI_RENDER_LAYER,
            GameplayAppState,
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
            ))
            .observe(|_: Trigger<Pointer<Click>>, mut next: ResMut<NextState<AppState>>|
            {
                next.set(AppState::Frontend);
            });
        });
    }
}

fn main()
{
    let ext = base::extents::Extents{ width: 10, height: 10 };
    let _arr = ext.neighbours::<{ base::extents::Neighbours::Top.bits() }>(base::extents::Point::new(0, 0));
    let _arr = ext.neighbours::<{ base::extents::Neighbours::Top.union(base::extents::Neighbours::Bottom).bits() }>(base::extents::Point::new(0, 0));
    let _ = find_assets_folder();

    base::hello_base();
    bevyx::hello_bevyx();
    sim::hello_sim();
    vis::hello_vis();

    App::new()
        .add_plugins(
            DefaultPlugins.set(RenderPlugin
            {
                render_creation: WgpuSettings
                {
                    backends: Some(Backends::DX12),
                    features: bevy::render::render_resource::WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }.into(),
                ..default()
            })
            .set(WindowPlugin
             {
                 exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
                ..default()
             }),
        )
        .add_plugins(FpsOverlayPlugin
        {
            config: FpsOverlayConfig
            {
                enabled: true,
                text_config: TextFont
                {
                    font_size: 20.0,
                    ..default()
                },
                ..default()
            },
        })
        .add_plugins(UiLunexPlugin)
        .add_plugins(UiLunexDebugPlugin::<{ layers::DEBUG_LAYER_2D }, { layers::DEBUG_LAYER_3D }>)
        .insert_state(AppState::Splash)
        .add_plugins(crate::debug::DebugPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(vis::GameVisPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, input::camera_pan)
        .add_systems(Update, input::camera_zoom)
        .add_systems(Update, input::reveal_cell)
        .add_systems(OnEnter(AppState::Splash), SplashAppState::spawn)
        .add_systems(OnExit(AppState::Splash), despawn_scene::<SplashAppState>)
        .add_systems(OnEnter(AppState::Frontend), FrontendAppState::spawn)
        .add_systems(OnExit(AppState::Frontend), despawn_scene::<FrontendAppState>)
        .add_systems(OnEnter(AppState::Gameplay), GameplayAppState::spawn)
        .add_systems(OnExit(AppState::Gameplay), despawn_scene::<GameplayAppState>)
        .run();
}

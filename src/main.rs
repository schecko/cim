
mod debug;
mod input;
mod layers;
mod screens;
mod app_state;
mod interactor;
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
                 primary_window: Some(Window
                 {
                    // present_mode: bevy::window::PresentMode::AutoNoVsync,
                    ..default()
                 }),
                ..default()
             })
             // .disable::<bevy::render::pipelined_rendering::PipelinedRenderingPlugin>(), // for novsync
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
                text_color: Color::BLACK,
                ..default()
            },
        })
        // .add_plugins(UiLunexPlugin::<{ layers::UI_LAYER }> )
        .add_plugins(UiLunexPlugin)
        .add_plugins(UiLunexDebugPlugin::<{ layers::DEBUG_LAYER_2D }, { layers::DEBUG_LAYER_3D }>)
        .insert_state(crate::app_state::AppState::Splash)
        .add_plugins(crate::debug::DebugPlugin)
        .add_plugins(EguiPlugin { enable_multipass_for_primary_context: true })
        .add_plugins(vis::GameVisPlugin)
        .add_plugins(app_state::splash::SplashAppState)
        .add_plugins(app_state::gameplay::GameplayAppState)
        .add_plugins(app_state::frontend::FrontendAppState)
        .add_systems(Startup, setup)
        .run();
}

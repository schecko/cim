
use ron::*;

use bevy::dev_tools::fps_overlay::FpsOverlayConfig;
use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::Backends;
use bevy::render::settings::WgpuSettings;

fn find_assets_folder() -> Result<(), std::io::Error>
{
    let mut current_dir = std::env::current_dir()?;

    while !current_dir.as_os_str().is_empty()
    {
        let assets_path = current_dir.join(bevyx::helper::ASSETS_FOLDER);
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
    (
        Camera2d::default()
    );
}

fn camera_pan
(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut previous_mouse_position: Local<Option<Vec2>>,
    windows: Query<&Window>,
)
{
    let window = windows.single();
    let mut camera_transform = camera_query.single_mut();
    let current_mouse_position = window.cursor_position();

    if mouse_buttons.pressed(MouseButton::Right)
    {
        if let (Some(current_pos), Some(previous_pos)) = (current_mouse_position, *previous_mouse_position)
        {
            let delta = current_pos - previous_pos;

            camera_transform.translation.x -= delta.x;
            camera_transform.translation.y += delta.y; // Y is inverted in screen space
        }

        // Update the previous mouse position
        *previous_mouse_position = current_mouse_position;
    }
    else
    {
        // Reset previous mouse position when the button is released
        *previous_mouse_position = None;
    }
}

fn camera_zoom
(
    mut ortho_query: Query<&mut OrthographicProjection, With<Camera2d>>,
    mut scroll_events: EventReader<MouseWheel>,
)
{
    use bevy::input::mouse::MouseScrollUnit;
    let mut ortho = ortho_query.single_mut();
    for event in scroll_events.read()
    {
        match event.unit
        {
            MouseScrollUnit::Line =>
            {
                ortho.scale -= event.y * 0.1;
            },
            MouseScrollUnit::Pixel =>
            {
                println!("Scroll (pixel units): vertical: {}, horizontal: {}", event.y, event.x);
            }
        }
    }
    ortho.scale = ortho.scale.clamp(0.5, 5.0);
}

fn main()
{
    let ext = base::extents::Extents{ width: 10, height: 10 };
    let _arr = ext.neighbours::<{ base::extents::Neighbours::Top.bits() }>( 0, 0 );
    let _arr = ext.neighbours::<{ base::extents::Neighbours::Top.union(base::extents::Neighbours::Bottom).bits() }>( 0, 0 );
    let _ = find_assets_folder();

    base::hello_base();
    bevyx::hello_bevyx();
    sim::hello_sim();
    vis::hello_vis();

    App::new()
        .add_plugins(
            DefaultPlugins.set(RenderPlugin {
                render_creation: WgpuSettings {
                    backends: Some(Backends::DX12),
                    ..default()
                }
                .into(),
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
        .add_plugins(vis::GameVisPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, camera_pan)
        .add_systems(Update, camera_zoom)
        .run();
}

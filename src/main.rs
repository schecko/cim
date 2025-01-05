
use bevy::dev_tools::fps_overlay::FpsOverlayConfig;
use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::camera::RenderTarget;
use bevy::render::settings::Backends;
use bevy::render::settings::WgpuSettings;
use bevy::window::PrimaryWindow;
use bevy::window::WindowRef;
use bevy_egui::EguiContext;
use bevy_egui::EguiPlugin;
use bevy_egui::egui;

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
    (
        Camera2d::default()
    );
}

fn camera_pan
(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
    mut previous_mouse_position: Local<Option<Vec2>>,
    windows: Query<&Window, With<PrimaryWindow>>,
)
{
    let Ok(window) = windows.get_single() else
    {
        return;
    };
    let Ok((mut camera_transform, projection)) = camera_query.get_single_mut() else
    {
        return;
    };
    let Some(current_mouse_pos) = window.cursor_position() else
    {
        return;
    };

    if mouse_buttons.pressed(MouseButton::Right)
    {
        if let Some(previous_mouse_pos) = *previous_mouse_position
        {
            let delta = current_mouse_pos - previous_mouse_pos;
            camera_transform.translation.x -= delta.x * projection.scale;
            camera_transform.translation.y += delta.y * projection.scale; // Y is inverted in screen space
        }

        *previous_mouse_position = Some(current_mouse_pos);
    }
    else
    {
        *previous_mouse_position = None;
    }
}

fn camera_zoom
(
    mut ortho_query: Query<&mut OrthographicProjection, With<Camera2d>>,
    mut scroll_events: EventReader<MouseWheel>,
)
{
    let Ok(mut ortho) = ortho_query.get_single_mut() else
    {
        return;
    };

    use bevy::input::mouse::MouseScrollUnit;
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
    ortho.scale = ortho.scale.clamp(0.01, 5.0);
}

fn create_new_window_system(mut commands: Commands)
{
    let second_window_id = commands
        .spawn(Window {
            title: "Dev window".to_owned(),
            // resolution: WindowResolution::new(800.0, 600.0),
            // present_mode: PresentMode::AutoVsync,
            ..Default::default()
        })
        .id();

    commands.spawn((
        Camera3d::default(),
        Camera {
            target: RenderTarget::Window(WindowRef::Entity(second_window_id)),
            ..Default::default()
        },
        Transform::from_xyz(6.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn primary_ui
(
    mut egui_ctx: Query<&mut EguiContext, With<PrimaryWindow>>,
)
{
    let Ok(mut ctx) = egui_ctx.get_single_mut() else {
        return;
    };
    egui::TopBottomPanel::top("top_panel")
        .resizable(false)
        .show(ctx.get_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Cim Debug");
                if ui.button("Hello").clicked()
                {
                    println!("Hello debug");
                }
            });
        });
}

fn dev_ui
(
    mut egui_ctx: Query<&mut EguiContext, Without<PrimaryWindow>>,
)
{
    let Ok(mut ctx) = egui_ctx.get_single_mut() else {
        return;
    };
    egui::Window::new("Hello").show(ctx.get_mut(), |ui| {
        ui.label("world");
    });
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
        .add_plugins(EguiPlugin)
        .add_plugins(vis::GameVisPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, create_new_window_system)
        .add_systems(Update, camera_pan)
        .add_systems(Update, camera_zoom)
        .add_systems(Update, (primary_ui, dev_ui))
        .run();
}

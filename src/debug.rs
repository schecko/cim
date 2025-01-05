
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::PrimaryWindow;
use bevy::window::WindowRef;
use bevy_egui::EguiContext;
use bevy_egui::EguiPlugin;
use bevy_egui::egui;

#[derive(Resource, Default)]
struct DebugState
{
    show_menu: bool,
}

fn setup(mut commands: Commands)
{
    let second_window_id = commands
        .spawn(Window {
            title: "Dev window".to_owned(),
            visible: false,
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

fn primary_window_ui
(
    keys: Res<ButtonInput<KeyCode>>,
    mut debug_state: ResMut<DebugState>,
    mut egui_ctx: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut secondary_window: Query<&mut Window, Without<PrimaryWindow>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>
)
{
    if keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) && keys.just_pressed(KeyCode::KeyC)
    {
        app_exit_events.send(AppExit::Success);
    }
    if keys.just_pressed(KeyCode::Backquote)
    {
        debug_state.show_menu = !debug_state.show_menu;
    }

    if !debug_state.show_menu
    {
        return;
    }
    let Ok(mut ctx) = egui_ctx.get_single_mut() else {
        return;
    };
    egui::TopBottomPanel::top("debug_panel")
        .resizable(false)
        .show(ctx.get_mut(), |ui|
        {
            ui.horizontal(|ui|
            {
                ui.label("Cim Debug");
                if ui.button("Dev Window").clicked()
                {
                    secondary_window.single_mut().visible = !secondary_window.single().visible;
                }
            });
        });
}

fn dev_window_ui
(
    keys: Res<ButtonInput<KeyCode>>,
    mut debug_state: ResMut<DebugState>,
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

pub struct DebugPlugin;

impl Plugin for DebugPlugin
{
    fn build(&self, app: &mut App)
    {
        app
            .insert_resource(DebugState::default())
            .add_systems(Startup, setup)
            .add_systems(Update, primary_window_ui)
            .add_systems(Update, dev_window_ui);
    }
}


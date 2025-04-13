
use crate::interactor::Interactor;

use vis::board_vis_tuning::BoardVisTuning;
use vis::grid_vis::GridVis;

use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Debug, Clone, Component)]
pub struct GameplayCamera;

pub fn camera_pan
(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), (With<Camera2d>, With<GameplayCamera>)>,
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

pub fn reveal_cell
(
    camera_query: Query<(&Camera, &GlobalTransform), (With<Camera2d>, With<GameplayCamera>)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut gizmos: Gizmos,
    board_vis_tuning: Res<BoardVisTuning>,
    mut interactor: ResMut<Interactor>,
    mut grid_vis: ResMut<GridVis>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
)
{
    let Ok((camera, camera_transform)) = camera_query.get_single() else
    {
        return;
    };

    let Ok(window) = windows.get_single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    gizmos.circle_2d(point, 10., bevy::color::palettes::basic::WHITE);

    if !mouse_buttons.just_pressed(MouseButton::Left)
    {
        return;
    }

    gizmos.circle_2d(point, 5., bevy::color::palettes::basic::RED);
    interactor.on_tap(&mut grid_vis.grid, &board_vis_tuning, &point);
}

pub fn camera_zoom
(
    mut ortho_query: Query<&mut OrthographicProjection, (With<Camera2d>, With<GameplayCamera>)>,
    mut scroll_events: EventReader<MouseWheel>,
    keys: Res<ButtonInput<KeyCode>>,
)
{
    let Ok(mut ortho) = ortho_query.get_single_mut() else
    {
        return;
    };

    if keys.just_pressed(KeyCode::KeyR)
    {
        ortho.scale = 1.0;
    }

    use bevy::input::mouse::MouseScrollUnit;
    for event in scroll_events.read()
    {
        match event.unit
        {
            MouseScrollUnit::Line =>
            {
                ortho.scale -= event.y * 0.1 * ortho.scale;
            },
            MouseScrollUnit::Pixel =>
            {
                println!("Scroll (pixel units): vertical: {}, horizontal: {}", event.y, event.x);
            }
        }
    }
    ortho.scale = ortho.scale.clamp(0.01, 5.0);
}


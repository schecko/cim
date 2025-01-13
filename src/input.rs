
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn camera_pan
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

pub fn camera_zoom
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


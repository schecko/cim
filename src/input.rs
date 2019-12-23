
use specs::prelude::*;
use glutin::ContextBuilder;
use glutin::event::{Event, WindowEvent, VirtualKeyCode, ElementState, KeyboardInput, };
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ PossiblyCurrent, };

pub enum InputMode {
    Normal,
    Edit,
    Command,
    Editor,
}


pub struct InputState {
    mode: InputMode,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            mode: InputMode::Normal,
        }
    }

    pub fn event(&mut self, world: &mut World, input: &KeyboardInput) {
        world.exec(|(mut game_state): (WriteExpect<crate::GameState>)| {
            let (grid_dim_x, grid_dim_y) = game_state.grid.dim();
            match input.virtual_keycode {
                Some(VirtualKeyCode::H) => {
                    if game_state.cursor.x < grid_dim_x - 1 {
                        game_state.cursor.x += 1;
                    }
                },
                Some(VirtualKeyCode::J) => {
                    if game_state.cursor.y < grid_dim_y - 1 {
                        game_state.cursor.y += 1;
                    }
                },
                Some(VirtualKeyCode::K) => {
                    if game_state.cursor.y > 0 {
                        game_state.cursor.y -= 1;
                    }
                },
                Some(VirtualKeyCode::L) => {
                    if game_state.cursor.x > 0 {
                        game_state.cursor.x -= 1;
                    }
                },
                _ => {},
            }
        });
    }
}




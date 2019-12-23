
use specs::prelude::*;
use glutin::ContextBuilder;
use glutin::event::{Event, WindowEvent, VirtualKeyCode, ElementState, KeyboardInput, ModifiersState, };
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ PossiblyCurrent, };
use strum::{EnumCount};

#[derive(EnumCount, Clone, Copy)]
pub enum InputMode {
    Normal,
    Edit,
    Command,
    Editor,
}

enum InputModeTransition {
    None,
    Reset,
    Go(InputMode),
}

struct Node {
    children: Vec<Node>,
    scancode: u32,
    modifiers: ModifiersState,
    action: Option<fn(&mut World) -> Option<InputMode>>,
}

impl Node {
    fn new() -> Self {
        Self {
            children: Vec::new(),
            scancode: 0,
            modifiers: Default::default(),
            action: None,
        }
    }

    fn add_child(&mut self, other: Node) {
        self.children.push(other);
    }

}

pub struct InputState {
    mode: InputMode,
    trees: [Node; INPUTMODE_COUNT],
    current: *mut Node,
}

impl InputState {
    pub fn new() -> Self {
        let mut normal = Node::new();
        let mut j_node = Node::new();
        let mut jj_node = Node::new();
        j_node.action = Some(|_| {
            println!("hello world");
            None
        });
        j_node.scancode = 36; // J

        jj_node.action = Some(|_| {
            println!("hello world 2");
            None
        });
        jj_node.scancode = 36; // J

        j_node.add_child(jj_node);
        normal.add_child(j_node);

        InputState {
            mode: InputMode::Normal,
            trees: [
                normal,
                Node::new(),
                Node::new(),
                Node::new(),
            ],
            current: std::ptr::null_mut(),
        }
    }

    unsafe fn traverse(&mut self, world: &mut World, input: &KeyboardInput) -> Option<InputMode> {
        match (*self.current).children.iter_mut().find(|node| node.scancode == input.scancode) {
            Some(child) => {
                self.current = child as *mut _;

                match (*self.current).children.len() {
                    0 => {
                        let transition = match (*self.current).action {
                            Some(fun) => fun(world),
                            None => None,
                        };
                        self.current = std::ptr::null_mut();
                        return transition;
                    },
                    _ => { },
                }
            },
            None => {
                self.current = std::ptr::null_mut();
            }
        }

        None
    }

    pub fn event(&mut self, world: &mut World, input: &KeyboardInput) {
        if self.current == std::ptr::null_mut() {
            self.current = &mut self.trees[self.mode as usize] as *mut _;
        }

        unsafe {
            self.traverse(world, input);
        }

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




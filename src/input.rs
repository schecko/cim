
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
    Camera,
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

static CAMERA_SPEED: f32 = 0.01;

impl InputState {
    pub fn new() -> Self {
        let normal = Node {
            action: None,
            modifiers: Default::default(),
            scancode: 0,
            children: vec![
                Node {
                    action: Some(|_| {
                        Some(InputMode::Camera)
                    }),
                    modifiers: Default::default(),
                    scancode: 33, // F
                    children: vec![ ],
                },
                Node {
                    action: Some(|mut world| {
                        world.exec(|(mut game_state): (WriteExpect<crate::GameState>)| {
                            let (grid_dim_x, grid_dim_y) = game_state.grid.dim();
                            if game_state.cursor.x < grid_dim_x - 1 {
                                game_state.cursor.x += 1;
                            }
                        });
                        None
                    }),
                    modifiers: Default::default(),
                    scancode: 35, // H
                    children: vec![
                    ],
                },
                Node {
                    action: Some(|mut world| {
                        world.exec(|(mut game_state): (WriteExpect<crate::GameState>)| {
                            let (grid_dim_x, grid_dim_y) = game_state.grid.dim();
                            if game_state.cursor.y < grid_dim_y - 1 {
                                game_state.cursor.y += 1;
                            }
                        });
                        None
                    }),
                    modifiers: Default::default(),
                    scancode: 36, // J
                    children: vec![ ],
                },
                Node {
                    action: Some(|mut world| {
                        world.exec(|(mut game_state): (WriteExpect<crate::GameState>)| {
                            let (grid_dim_x, grid_dim_y) = game_state.grid.dim();
                            if game_state.cursor.y > 0 {
                                game_state.cursor.y -= 1;
                            }
                        });
                        None
                    }),
                    modifiers: Default::default(),
                    scancode: 37, // K
                    children: vec![ ],
                },
                Node {
                    action: Some(|mut world| {
                        world.exec(|(mut game_state): (WriteExpect<crate::GameState>)| {
                            let (grid_dim_x, grid_dim_y) = game_state.grid.dim();
                            if game_state.cursor.x > 0 {
                                game_state.cursor.x -= 1;
                            }
                        });
                        None
                    }),
                    modifiers: Default::default(),
                    scancode: 38, // L
                    children: vec![ ],
                },
                Node {
                    action: Some(|_| {
                        Some(InputMode::Command)
                    }),
                    modifiers: ModifiersState {
                        shift: true,
                        ..Default::default()
                    },
                    scancode: 39, // ;
                    children: vec![ ],
                },
            ],
        };

        let camera = Node {
            action: None,
            modifiers: Default::default(),
            scancode: 0,
            children: vec![
                Node {
                    action: Some(|mut world| {
                        world.exec(|(mut camera): (WriteExpect<crate::Camera>)| {
                            camera.view.rot.s += CAMERA_SPEED;
                        });
                        None
                    }),
                    modifiers: Default::default(),
                    scancode: 35, // H
                    children: vec![
                    ],
                },
                Node {
                    action: Some(|mut world| {
                        world.exec(|(mut camera): (WriteExpect<crate::Camera>)| {
                            camera.view.rot.s -= CAMERA_SPEED;
                        });
                        None
                    }),
                    modifiers: ModifiersState {
                        shift: true,
                        ..Default::default()
                    },
                    scancode: 35, // H
                    children: vec![
                    ],
                },
                Node {
                    action: Some(|mut world| {
                        world.exec(|(mut camera): (WriteExpect<crate::Camera>)| {
                            camera.view.rot.v.x += CAMERA_SPEED;
                        });
                        None
                    }),
                    modifiers: Default::default(),
                    scancode: 36, // J
                    children: vec![ ],
                },
                Node {
                    action: Some(|mut world| {
                        world.exec(|(mut camera): (WriteExpect<crate::Camera>)| {
                            camera.view.rot.v.x -= CAMERA_SPEED;
                        });
                        None
                    }),
                    modifiers: ModifiersState {
                        shift: true,
                        ..Default::default()
                    },
                    scancode: 36, // J
                    children: vec![ ],
                },
                Node {
                    action: Some(|mut world| {
                        world.exec(|(mut camera): (WriteExpect<crate::Camera>)| {
                            camera.view.rot.v.y += CAMERA_SPEED;
                        });
                        None
                    }),
                    modifiers: Default::default(),
                    scancode: 37, // K
                    children: vec![ ],
                },
                Node {
                    action: Some(|mut world| {
                        world.exec(|(mut camera): (WriteExpect<crate::Camera>)| {
                            camera.view.rot.v.y -= CAMERA_SPEED;
                        });
                        None
                    }),
                    modifiers: ModifiersState {
                        shift: true,
                        ..Default::default()
                    },
                    scancode: 37, // K
                    children: vec![ ],
                },
                Node {
                    action: Some(|mut world| {
                        world.exec(|(mut camera): (WriteExpect<crate::Camera>)| {
                            camera.view.rot.v.z -= CAMERA_SPEED;
                        });
                        None
                    }),
                    modifiers: Default::default(),
                    scancode: 38, // L
                    children: vec![ ],
                },
                Node {
                    action: Some(|mut world| {
                        world.exec(|(mut camera): (WriteExpect<crate::Camera>)| {
                            camera.view.rot.v.z -= 0.1;
                        });
                        None
                    }),
                    modifiers: ModifiersState {
                        shift: true,
                        ..Default::default()
                    },
                    scancode: 38, // L
                    children: vec![ ],
                },
            ]
        };

        let command = Node {
            action: None,
            modifiers: Default::default(),
            scancode: 0,
            children: vec![
                Node {
                    action: Some(|_| {
                        Some(InputMode::Normal)
                    }),
                    modifiers: Default::default(),
                    scancode: 1, // <ESC>
                    children: vec![ ],
                },
            ]
        };

        InputState {
            mode: InputMode::Normal,
            trees: [
                normal,
                Node::new(),
                command,
                Node::new(),
                camera,
            ],
            current: std::ptr::null_mut(),
        }
    }

    unsafe fn traverse(&mut self, world: &mut World, input: &KeyboardInput) {
        match (*self.current).children.iter_mut().find(|node| node.scancode == input.scancode && node.modifiers == input.modifiers) {
            Some(child) => {
                self.current = child as *mut _;

                match (*self.current).children.len() {
                    0 => {
                        let transition = match (*self.current).action {
                            Some(fun) => fun(world),
                            None => None,
                        };
                        if let Some(new_mode) = transition {
                            self.mode = new_mode;
                        }

                        self.current = std::ptr::null_mut();
                    },
                    _ => { },
                }
            },
            None => {
                self.current = std::ptr::null_mut();
            }
        }
    }

    pub fn event(&mut self, world: &mut World, input: &KeyboardInput) {
        if input.scancode == 1 {
            self.mode = InputMode::Normal;
            self.current = std::ptr::null_mut();
            return;
        }

        if self.current == std::ptr::null_mut() {
            self.current = &mut self.trees[self.mode as usize] as *mut _;
        }

        unsafe {
            self.traverse(world, input);
        }
    }
}





use specs::prelude::*;
use glutin::ContextBuilder;
use glutin::event::{Event, WindowEvent, VirtualKeyCode, ElementState, KeyboardInput, ModifiersState, ScanCode, };
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

#[derive(Debug)]
pub enum KeyCode {
    Virtual(VirtualKeyCode),
    Physical(ScanCode),
}

impl Default for KeyCode {
    fn default() -> Self {
        KeyCode::Physical(0)
    }
}

struct Node {
    children: Vec<Node>,
    code: KeyCode,
    modifiers: ModifiersState,
    action: Option<fn(&mut World) -> Option<InputMode>>,
}

impl Node {
    fn new() -> Self {
        Self {
            children: Vec::new(),
            code: KeyCode::Physical(0),
            modifiers: Default::default(),
            action: None,
        }
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
            code: Default::default(), // root is unused
            modifiers: Default::default(),
            children: vec![
                Node {
                    action: Some(|_| {
                        Some(InputMode::Camera)
                    }),
                    modifiers: Default::default(),
                    code: KeyCode::Virtual(VirtualKeyCode::F),
                    children: vec![ ],
                },
                Node {
                    action: Some(|_| {
                        Some(InputMode::Edit)
                    }),
                    modifiers: Default::default(),
                    code: KeyCode::Virtual(VirtualKeyCode::I),
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
                    code: KeyCode::Virtual(VirtualKeyCode::H),
                    children: vec![ ],
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
                    code: KeyCode::Virtual(VirtualKeyCode::J),
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
                    code: KeyCode::Virtual(VirtualKeyCode::K),
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
                    code: KeyCode::Virtual(VirtualKeyCode::L),
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
                    code: KeyCode::Virtual(VirtualKeyCode::Semicolon),
                    children: vec![ ],
                },
            ],
        };

        let camera = Node {
            action: None,
            modifiers: Default::default(),
            code: Default::default(), // root is unused
            children: vec![
                Node {
                    action: Some(|mut world| {
                        world.exec(|(mut camera): (WriteExpect<crate::Camera>)| {
                            camera.view.rot.s += CAMERA_SPEED;
                        });
                        None
                    }),
                    modifiers: Default::default(),
                    code: KeyCode::Virtual(VirtualKeyCode::H),
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
                    code: KeyCode::Virtual(VirtualKeyCode::H),
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
                    code: KeyCode::Virtual(VirtualKeyCode::J),
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
                    code: KeyCode::Virtual(VirtualKeyCode::J),
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
                    code: KeyCode::Virtual(VirtualKeyCode::K),
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
                    code: KeyCode::Virtual(VirtualKeyCode::K),
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
                    code: KeyCode::Virtual(VirtualKeyCode::L),
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
                    code: KeyCode::Virtual(VirtualKeyCode::L),
                    children: vec![ ],
                },
            ]
        };

        let command = Node {
            action: None,
            modifiers: Default::default(),
            code: Default::default(), // root is unused
            children: vec![
                Node {
                    action: Some(|_| {
                        Some(InputMode::Normal)
                    }),
                    modifiers: Default::default(),
                    code: KeyCode::Virtual(VirtualKeyCode::Escape),
                    children: vec![ ],
                },
                Node {
                    action: None,
                    modifiers: Default::default(),
                    code: KeyCode::Virtual(VirtualKeyCode::Q),
                    children: vec![
                        Node {
                            action: Some(|mut world| {
                                world.exec(|(mut game_state): (WriteExpect<crate::GameState>)| {
                                    game_state.running = false;
                                });
                                None
                            }),
                            modifiers: Default::default(),
                            code: KeyCode::Virtual(VirtualKeyCode::Return),
                            children: vec![ ],
                        },
                    ],
                },
            ]
        };

        let edit = Node {
            action: None,
            modifiers: Default::default(),
            code: Default::default(), // root is unused
            children: vec![
                Node {
                    action: Some(|mut world| {
                        world.exec(|(mut grid_pos, game_state): (WriteStorage<crate::GridPosition>, ReadExpect<crate::GameState>)| {
                            unimplemented!();
                            let (grid_dim_x, grid_dim_y) = game_state.grid.dim();
                            if game_state.cursor.x < grid_dim_x - 1 {
                                game_state.cursor.x += 1;
                            }
                        });
                        None
                    }),
                    modifiers: Default::default(),
                    code: KeyCode::Virtual(VirtualKeyCode::H),
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
        match (*self.current)
            .children
            .iter_mut()
            .find(|node| {
                (match node.code {
                    KeyCode::Virtual(v) => input.virtual_keycode.map_or(false, |iv| iv == v),
                    KeyCode::Physical(p) => input.scancode == p,
                })
                && node.modifiers == input.modifiers
            })
        {
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
        if input.virtual_keycode.map_or(false, |v| v == VirtualKeyCode::Escape) {
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




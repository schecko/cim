
use glutin::event::{ VirtualKeyCode, KeyboardInput, ModifiersState, ScanCode, };
use cgmath::prelude::*;
use crate::*;
use crate::game_state::*;

#[derive(Display, EnumCount, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Unit,
    Command,
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

pub struct InputState {
    current: *mut Node,
    pub mode: InputMode,
    pub modifiers: ModifiersState,
    trees: [Node; INPUTMODE_COUNT],
}

static CAMERA_SPEED: f32 = 0.01;
static LONG_MOVE: usize = 5;
static SHORT_MOVE: usize = 1;

impl InputState {
    pub fn new() -> Self {
        let normal = Node {
            action: None,
            code: Default::default(), // root is unused
            modifiers: Default::default(),
            children: vec![
                Node {
                    action: Some(|world| {
                        let player: *mut Player = &mut world.game_state.players[0] as *mut _;
                        for _ in 0..2 {
                            unsafe {
                                match &mut (*player).camera_jump {
                                    CameraJump::Unit(i) => {
                                        if (*player).turn_units.len() <= *i {
                                            (*player).camera_jump = CameraJump::Structure(0);
                                            continue;
                                        }

                                        let eid = (*player).turn_units[*i];
                                        *i += 1;
                                        let unit = &world.game_state.get_unit(eid);
                                        if let Some(u) = unit {
                                            world.game_state.cursor = u.loc;
                                            break;
                                        }
                                    },
                                    CameraJump::Structure(i) => {
                                        if (*player).turn_structures.len() <= *i {
                                            (*player).camera_jump = CameraJump::Unit(0);
                                            continue;
                                        }

                                        let eid = (*player).turn_structures[*i];
                                        *i += 1;
                                        let structure = world.game_state.get_structure(eid);
                                        if let Some(s) = structure {
                                            world.game_state.cursor = s.loc;
                                            break;
                                        }
                                    },
                                    _ => {
                                    },
                                }
                            }
                        }

                        None
                    }),
                    modifiers: Default::default(),
                    code: KeyCode::Virtual(VirtualKeyCode::Space),
                    children: vec![ ],
                },
                Node {
                    action: Some(|_| {
                        Some(InputMode::Camera)
                    }),
                    modifiers: Default::default(),
                    code: KeyCode::Virtual(VirtualKeyCode::F),
                    children: vec![ ],
                },
                Node {
                    action: Some(|world| {
                        // force end of turn for player
                        world.game_state.check_turn_complete(true);
                        None
                    }),
                    modifiers: Default::default(),
                    code: KeyCode::Virtual(VirtualKeyCode::T),
                    children: vec![ ],
                },
                Node {
                    action: Some(|world| {
                        let cell = world.game_state.get_grid(world.game_state.cursor);
                        if cell.unit.is_some() || cell.structure.is_some() {
                            Some(InputMode::Unit)
                        } else {
                            println!("No unit or structure to interact with");
                            None
                        }
                    }),
                    modifiers: Default::default(),
                    code: KeyCode::Virtual(VirtualKeyCode::I),
                    children: vec![ ],
                },
                Node {
                    action: Some(|world| {
                        world.game_state.yanked_location = Some(world.game_state.cursor);
                        None
                    }),
                    modifiers: Default::default(),
                    code: KeyCode::Virtual(VirtualKeyCode::Y),
                    children: vec![ ],
                },
                Node {
                    action: Some(|world| {
                        if let Some(loc) = world.game_state.yanked_location {
                            let source_unit = world.game_state.get_grid_mut(loc).unit.take();
                            let cursor = world.game_state.cursor;
                            let dest = world.game_state.get_grid_mut(cursor);

                            dest.unit = source_unit;
                        }
                        None
                    }),
                    modifiers: Default::default(),
                    code: KeyCode::Virtual(VirtualKeyCode::P),
                    children: vec![ ],
                },
                Node {
                    action: Some(|world| {
                        world.game_state.cursor = world.game_state.cursor.left(&world.game_state.grid, SHORT_MOVE).0;
                        None
                    }),
                    modifiers: Default::default(),
                    code: KeyCode::Virtual(VirtualKeyCode::H),
                    children: vec![ ],
                },
                Node {
                    action: Some(|world| {
                        world.game_state.cursor = world.game_state.cursor.left(&world.game_state.grid, LONG_MOVE).0;
                        None
                    }),
                    modifiers: ModifiersState::SHIFT,
                    code: KeyCode::Virtual(VirtualKeyCode::H),
                    children: vec![ ],
                },
                Node {
                    action: Some(|world| {
                        world.game_state.cursor = world.game_state.cursor.down(&world.game_state.grid, SHORT_MOVE).0;
                        None
                    }),
                    modifiers: Default::default(),
                    code: KeyCode::Virtual(VirtualKeyCode::J),
                    children: vec![ ],
                },
                Node {
                    action: Some(|world| {
                        world.game_state.cursor = world.game_state.cursor.down(&world.game_state.grid, LONG_MOVE).0;
                        None
                    }),
                    modifiers: ModifiersState::SHIFT,
                    code: KeyCode::Virtual(VirtualKeyCode::J),
                    children: vec![ ],
                },
                Node {
                    action: Some(|world| {
                        world.game_state.cursor = world.game_state.cursor.up(&world.game_state.grid, SHORT_MOVE).0;
                        None
                    }),
                    modifiers: Default::default(),
                    code: KeyCode::Virtual(VirtualKeyCode::K),
                    children: vec![ ],
                },
                Node {
                    action: Some(|world| {
                        world.game_state.cursor = world.game_state.cursor.up(&world.game_state.grid, LONG_MOVE).0;
                        None
                    }),
                    modifiers: ModifiersState::SHIFT,
                    code: KeyCode::Virtual(VirtualKeyCode::K),
                    children: vec![ ],
                },
                Node {
                    action: Some(|world| {
                        world.game_state.cursor = world.game_state.cursor.right(&world.game_state.grid, SHORT_MOVE).0;
                        None
                    }),
                    modifiers: Default::default(),
                    code: KeyCode::Virtual(VirtualKeyCode::L),
                    children: vec![ ],
                },
                Node {
                    action: Some(|world| {
                        world.game_state.cursor = world.game_state.cursor.right(&world.game_state.grid, LONG_MOVE).0;
                        None
                    }),
                    modifiers: ModifiersState::SHIFT,
                    code: KeyCode::Virtual(VirtualKeyCode::L),
                    children: vec![ ],
                },
                Node {
                    action: Some(|_| {
                        Some(InputMode::Command)
                    }),
                    modifiers: ModifiersState::SHIFT,
                    // Some keyboards return Semicolon with shift,
                    // others return Colon with shift
                    code: KeyCode::Virtual(VirtualKeyCode::Semicolon),
                    children: vec![ ],
                },
                Node {
                    action: Some(|_| {
                        Some(InputMode::Command)
                    }),
                    modifiers: ModifiersState::SHIFT,
                    // Some keyboards return Semicolon with shift,
                    // others return Colon with shift
                    code: KeyCode::Virtual(VirtualKeyCode::Colon),
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
                    action: Some(|world| {
                        let mut rot = Quaternion::<f32>::one();
                        rot.v.z = -CAMERA_SPEED;
                        world.camera.view.rot = world.camera.view.rot * rot;
                        world.camera.view.rot.normalize();
                        None
                    }),
                    modifiers: Default::default(),
                    code: KeyCode::Virtual(VirtualKeyCode::H),
                    children: vec![
                    ],
                },
                Node {
                    action: Some(|world| {
                        let mut rot = Quaternion::<f32>::one();
                        rot.v.x = CAMERA_SPEED;
                        world.camera.view.rot = world.camera.view.rot * rot;
                        world.camera.view.rot.normalize();
                        None
                    }),
                    modifiers: Default::default(),
                    code: KeyCode::Virtual(VirtualKeyCode::J),
                    children: vec![ ],
                },
                Node {
                    action: Some(|world| {
                        let mut rot = Quaternion::<f32>::one();
                        rot.v.x = -CAMERA_SPEED;
                        world.camera.view.rot = world.camera.view.rot * rot;
                        world.camera.view.rot.normalize();
                        None
                    }),
                    modifiers: Default::default(),
                    code: KeyCode::Virtual(VirtualKeyCode::K),
                    children: vec![ ],
                },
                Node {
                    action: Some(|world| {
                        let mut rot = Quaternion::<f32>::one();
                        rot.v.z = CAMERA_SPEED;
                        world.camera.view.rot = world.camera.view.rot * rot;
                        world.camera.view.rot.normalize();
                        None
                    }),
                    modifiers: Default::default(),
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
                    action: Some(|world| {
                        // check if the command command text has a valid command
                        // execute the command if it is valid
                        // return to normal mode, regardless of error or not
                        // NOTE: first char will always be a colon.
                        match &world.game_state.command_text[1..] {
                            "q" => world.game_state.running = false,
                            "add unit" => {
                                let unit = Unit::new(UnitType::Settler, 0.into(), world.game_state.cursor.loc);
                                world.game_state.add_unit(unit);
                            },
                            "add structure" => {
                                let s = Structure::new(world.game_state.turn, 0.into(), world.game_state.cursor.loc);
                                world.game_state.add_structure(s);
                            },
                            _ => {
                                println!("unknown command");
                            }
                        }
                        Some(InputMode::Normal)
                    }),
                    modifiers: Default::default(),
                    code: KeyCode::Virtual(VirtualKeyCode::Return),
                    // note, do not give this node children.
                    // command mode must parse the command_text of game_state for instruction
                    children: vec![ ],
                },
            ]
        };

        fn move_one(world: &mut World, direction: Vector2<isize>) {
            let cursor = world.game_state.cursor;
            let (dest_i, actual_translation) = cursor.translate(&world.game_state.grid, direction);
            let source_cell: *mut GridCell = world.game_state.get_grid_mut(cursor) as *mut _;
            let actual_distance = (num::abs(actual_translation.x) + num::abs(actual_translation.y)) as usize;

            unsafe {
                if let Some(uid) = (*source_cell).unit {
                    if let Some(unit) = world.game_state.get_unit_mut(uid) {
                        if unit.moves_remaining >= actual_distance {
                            unit.loc = dest_i;
                            unit.moves_remaining -= actual_distance;
                            if unit.moves_remaining <= 0 {
                                println!("Unit out of moves for this turn");
                                // this unit is finished moving for this turn, update turn_units
                                match world.game_state.players[0].turn_units.iter().position(|&eid| { Some(eid) == (*source_cell).unit }) {
                                    Some(i) => { world.game_state.players[0].turn_units.remove(i); },
                                    None => {},
                                }
                            }

                            let dest_cell = world.game_state.get_grid_mut(dest_i);
                            dest_cell.unit = (*source_cell).unit.take();
                            world.game_state.cursor = dest_i;
                        }
                    }
                }
            }
        }

        let unit = Node {
            action: None,
            modifiers: Default::default(),
            code: Default::default(), // root is unused
            children: vec![
                Node {
                    action: Some(|world| {
                        move_one(world, Vector2::new(-1, 0));
                        None
                    }),
                    modifiers: Default::default(),
                    code: KeyCode::Virtual(VirtualKeyCode::H),
                    children: vec![ ],
                },
                Node {
                    action: Some(|world| {
                        move_one(world, Vector2::new(0, -1));
                        None
                    }),
                    modifiers: Default::default(),
                    code: KeyCode::Virtual(VirtualKeyCode::J),
                    children: vec![ ],
                },
                Node {
                    action: Some(|world| {
                        move_one(world, Vector2::new(0, 1));
                        None
                    }),
                    modifiers: Default::default(),
                    code: KeyCode::Virtual(VirtualKeyCode::K),
                    children: vec![ ],
                },
                Node {
                    action: Some(|world| {
                        move_one(world, Vector2::new(1, 0));
                        None
                    }),
                    modifiers: Default::default(),
                    code: KeyCode::Virtual(VirtualKeyCode::L),
                    children: vec![ ],
                },
                Node {
                    action: Some(|world| {
                        // turn settler into a structure
                        let cursor = world.game_state.cursor;
                        let cell = world.game_state.get_grid(cursor);

                        if let (None, Some(uid)) = (cell.structure, cell.unit) {
                           if let Some(unit) = world.game_state.get_unit(uid) {
                                if unit.t == UnitType::Settler {
                                    let structure = Structure {
                                        next_unit_ready: world.game_state.turn + 5,
                                        next_unit: UnitType::Settler,
                                        loc: unit.loc,
                                        player: unit.player,
                                    };

                                    world.game_state.delete_unit(uid);
                                    world.game_state.add_structure(structure);
                                }
                            }
                        }
                        None
                    }),
                    modifiers: Default::default(),
                    code: KeyCode::Virtual(VirtualKeyCode::S),
                    children: vec![ ],
                },
            ]
        };

        InputState {
            mode: InputMode::Normal,
            trees: [
                normal,
                unit,
                command,
                camera,
            ],
            current: std::ptr::null_mut(),
            modifiers: Default::default(),
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
                && node.modifiers == self.modifiers
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
                        if self.mode != InputMode::Command {
                            world.game_state.command_text = String::new();
                        }
                    },
                    _ => { },
                }
            },
            None => {
                self.current = std::ptr::null_mut();
                if self.mode != InputMode::Command {
                    world.game_state.command_text = String::new();
                }
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




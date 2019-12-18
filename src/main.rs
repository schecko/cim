
#[macro_use] extern crate rand_derive;

extern crate cgmath;
extern crate glutin;
extern crate ndarray;
extern crate rand;

mod pipeline;

use ndarray::*;
use cgmath::prelude::*;
use cgmath::*;
use gl::types::*;
use glutin::ContextBuilder;
use glutin::event::{Event, WindowEvent, VirtualKeyCode, ElementState, };
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ PossiblyCurrent, };
use pipeline::*;
use std::mem;
use std::ffi::{ CString, CStr, };

static DEFAULT_GRID_LENGTH: usize = 4;

#[derive(Debug, Clone, Rand)]
enum Biome {
    Desert,
    Grassland,
    Hill,
    Mountain,
    Ocean,
    Snow,
}

impl Biome {
    fn color(&self) -> Vector3<f32> {
        match *self {
            Biome::Desert => Vector3::new(1.0, 1.0, 0.7),
            Biome::Grassland => Vector3::new(0.0, 1.0, 0.0),
            Biome::Hill => Vector3::new(1.0, 1.0, 0.7),
            Biome::Mountain => Vector3::new(1.0, 1.0, 1.0),
            Biome::Ocean => Vector3::new(0.0, 0.0, 1.0),
            Biome::Snow => Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

#[derive(Debug, Clone)]
struct GridCell {
    pub biome: Biome,
}

struct GameState {
    grid: Array2<GridCell>,
    cursor: Vector2<usize>,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            grid: Array2::from_shape_fn((20, 20), |(x, y)| GridCell { biome: rand::random() }),
            cursor: Vector2::new(0, 0),
        }
    }
}

fn load(context: &glutin::Context<PossiblyCurrent>) {
    gl::load_with(|ptr| context.get_proc_address(ptr) as *const _);

    let version = unsafe {
        CStr::from_ptr(gl::GetString(gl::VERSION) as *const _).to_str().unwrap()
    };

    println!("Opengl Version: {}", version);
}

static RECT: [[f32; 3]; 6] = [
    [1.0, 1.0, 0.0],
    [-1.0, 1.0, 0.0],
    [1.0, -1.0, 0.0],

    [-1.0, -1.0, 0.0],
    [1.0, -1.0, 0.0],
    [-1.0, 1.0, 0.0],
];

static VERTEX: &str = r#"
    #version 330 core

    layout (location = 0) in vec3 aVertOffset;
    layout (location = 1) in vec3 aWorldPos;
    layout (location = 2) in vec3 aColor;

    out vec3 fColor;

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 proj;

    void main() {
        gl_Position = proj * view * model * vec4(aWorldPos + aVertOffset, 1.0);
        fColor = aColor;
    }
"#;

static FRAGMENT: &str = r#"
    #version 330 core

    out vec4 FragColor;
    in vec3 fColor;

    void main() {
        FragColor = vec4(fColor, 1.0);
    }
"#;

fn main() -> Result<(), String> {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new().with_title("Cim");

    let context = ContextBuilder::new()
        .build_windowed(window_builder, &event_loop)
        .unwrap();

    let context = unsafe { context.make_current().unwrap() };

    load(context.context());
    let solid = Pipeline::new(VERTEX, FRAGMENT)?;

    let mut game_state = GameState::new();

    let (grid_width, grid_height) = game_state.grid.dim();
    let mut rect_positions: Vec<_> = game_state.grid.indexed_iter().map(|((x, y), grid)| {
        [
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 0.0),
        ]
    }).collect();

    let (vao, quad_vbo, instance_vbo) = unsafe {
        let mut vao: u32 = 0;
        gl::GenVertexArrays(1, &mut vao as *mut _);
        let mut quad_vbo: u32 = 0;
        gl::GenBuffers(1, &mut quad_vbo as *mut _);
        let mut instance_vbo: u32 = 0;
        gl::GenBuffers(1, &mut instance_vbo as *mut _);

        gl::BindBuffer(gl::ARRAY_BUFFER, instance_vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (rect_positions.len() * mem::size_of_val(&rect_positions[0])) as isize, rect_positions.as_mut_ptr() as *mut _, gl::STATIC_DRAW);

        gl::BindVertexArray(vao);
        assert!(mem::size_of::<f32>() * 3 == mem::size_of::<Vector3<f32>>());
        assert!(mem::size_of::<f32>() * 4 == mem::size_of::<Vector4<f32>>());

        gl::BindBuffer(gl::ARRAY_BUFFER, quad_vbo);
        gl::BufferData(gl::ARRAY_BUFFER, mem::size_of_val(&RECT) as isize, RECT.as_ptr() as *mut _, gl::STATIC_DRAW);
        assert!(gl::GetError() == 0);


        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<f32>() as i32, std::ptr::null());

        gl::BindBuffer(gl::ARRAY_BUFFER, instance_vbo);
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 6 * mem::size_of::<f32>() as i32, std::ptr::null());
        gl::VertexAttribDivisor(1, 1);
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, 6 * mem::size_of::<f32>() as i32, (3 * mem::size_of::<f32>()) as *const _);
        gl::VertexAttribDivisor(2, 1);
        assert!(gl::GetError() == 0);

        // reset opengl state
        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        (vao, quad_vbo, instance_vbo)
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => {
                match event {
                    WindowEvent::Resized(logical_size) => {
                        let dpi_factor = context.window().hidpi_factor();
                        let physical = logical_size.to_physical(dpi_factor);
                        context.resize(physical);
                        unsafe { gl::Viewport(0, 0, physical.width as _, physical.height as _); }
                    },
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input: glutin::event::KeyboardInput { state, scancode, virtual_keycode, modifiers }, ..} if *state == ElementState::Pressed => {
                        let (grid_dim_x, grid_dim_y) = game_state.grid.dim();
                        match virtual_keycode {
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

                    },
                    _ => {},
                }
            },
            _ => { },
        };


        rect_positions = game_state.grid.indexed_iter().map(|((x, y), grid)| {
            let loc_z = match game_state.cursor == Vector2::new(x, y) {
                true => 1.0,
                false => 0.0,
            };
            let loc_x = (grid_width as isize / 2 - x as isize) as f32;
            let loc_y = (grid_height as isize/ 2 - y as isize) as f32;
            // TODO: game grid lines rather than spacers.
            [
                Vector3::new(2.2 * loc_x, 2.2 * loc_y, loc_z),
                grid.biome.color(),
            ]
        }).collect();

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, instance_vbo);
            gl::BufferSubData(gl::ARRAY_BUFFER, 0, (rect_positions.len() * mem::size_of_val(&rect_positions[0])) as isize, rect_positions.as_mut_ptr() as *mut _);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            gl::ClearColor(1.0, 0.5, 0.7, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            solid.set_use();

            let decomp = Decomposed {
                scale: 1.0,
                rot: Quaternion::new(1.0, -0.5, 0.0, 0.0),
                disp: Vector3::new(0.0, 0.0, 0.0),
            };

            // TODO: proper screen ratio
            let proj: Matrix4<f32> = perspective(Deg(45.0), 1.0, 0.1, 1000.0);
            let view: Matrix4<f32> = Decomposed {
                scale: 1.0,
                rot: Quaternion::new(0.0f32, 0.0, 0.0, 0.0),
                disp: Vector3::new(0.0f32, 0.0, -60.0),
            }.into();

            let model: Matrix4<f32> = decomp.into();

            let model_loc = solid.get_uniform_location("model");
            let view_loc = solid.get_uniform_location("view");
            let proj_loc = solid.get_uniform_location("proj");

            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.as_ptr());
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view.as_ptr());
            gl::UniformMatrix4fv(proj_loc, 1, gl::FALSE, proj.as_ptr());

            assert!(gl::GetError() == 0);
            gl::BindVertexArray(vao);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            gl::DrawArraysInstanced(gl::TRIANGLES, 0, RECT.len() as i32, rect_positions.len() as i32);

            //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            //gl::DrawArraysInstanced(gl::TRIANGLES, 0, RECT.len() as i32, rect_positions.len() as i32);
            assert!(gl::GetError() == 0);
        }
        context.swap_buffers().unwrap();
    });

}

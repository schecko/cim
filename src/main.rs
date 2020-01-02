
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate rand_derive;
#[macro_use] extern crate specs;
#[macro_use] extern crate strum_macros;
extern crate cgmath;
extern crate glutin;
extern crate ndarray;
extern crate rand;
extern crate strum;
extern crate num;

mod pipeline;
mod renderer;
mod ogl;
mod input;

use cgmath::*;
use cgmath::prelude::*;
use gl::types::*;
use glutin::ContextBuilder;
use glutin::event::{Event, WindowEvent, VirtualKeyCode, ElementState, };
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ PossiblyCurrent, };
use ndarray::*;
use pipeline::*;
use specs::prelude::*;
use std::ffi::{ CString, CStr, };
use std::mem;
use crate::renderer::*;
use crate::ogl::*;
use crate::input::*;
use rand::distributions::{ Uniform, Distribution, };

static DEFAULT_GRID_LENGTH: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq, Rand)]
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
    pub unit: Option<specs::world::Index>,
}

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct GridPosition {
    xy: (usize, usize),
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
struct Cursor {
    pub loc: (usize, usize),
}

impl Cursor {
    fn new(x: usize, y: usize) -> Self {
        Self { loc: (x, y) }
    }

    fn left<T>(&self, grid: &Array2<T>, distance: usize) -> Self {
        let (grid_dim_x, grid_dim_y) = grid.dim();
        let new_vert = self.loc.0 as isize - distance as isize;
        Cursor{ loc: (num::clamp(new_vert, 0, grid_dim_x as isize - 1) as usize, self.loc.1) }
    }

    fn right<T>(&self, grid: &Array2<T>, distance: usize) -> Self {
        let (grid_dim_x, grid_dim_y) = grid.dim();
        let new_vert = self.loc.0 + distance;
        Cursor{ loc: (num::clamp(new_vert, 0, grid_dim_x - 1), self.loc.1) }
    }

    fn up<T>(&self, grid: &Array2<T>, distance: usize) -> Self {
        let (grid_dim_x, grid_dim_y) = grid.dim();
        let new_vert = self.loc.1 + distance;
        Cursor{ loc: (self.loc.0, num::clamp(new_vert, 0, grid_dim_y - 1)) }
    }

    fn down<T>(&self, grid: &Array2<T>, distance: usize) -> Self {
        let (grid_dim_x, grid_dim_y) = grid.dim();
        let new_vert = self.loc.1 as isize - distance as isize;
        Cursor{ loc: (self.loc.0, num::clamp(new_vert, 0, grid_dim_y as isize - 1) as usize) }
    }
}

impl From<(usize, usize)> for Cursor {
    fn from(other: (usize, usize)) -> Self {
        Self { loc: other }
    }
}

#[derive(Component)]
pub struct GameState {
    grid: Array2<GridCell>,
    cursor: Cursor,
    solid: Pipeline,

    quad_data: Buffer,
    quad_instance_data: Buffer,
    quad_vao: Vao,

    cube_data: Buffer,
    cube_instance_data: Buffer,
    cube_vao: Vao,

    running: bool,

    yanked_location: Option<Cursor>,
}

#[derive(Component)]
pub struct Camera {
    projection: Matrix4<f32>,
    view: Decomposed<Vector3<f32>, Quaternion<f32>>,
}

impl Camera {
    fn new() -> Self {
        Self {
            projection: perspective(Deg(45.0), 1.0, 0.1, 1000.0),
            view: Decomposed {
                scale: 1.0,
                rot: Quaternion::new(-0.6f32, 0.50, 0.0, 0.0),
                disp: Vector3::new(0.0f32, 0.0, -60.0),
            },
        }
    }
}

fn grid_fill(mut grid: &mut Array2<GridCell>, depth: u32, max_depth: u32, (x, y): (usize, usize)) {
    let cell = grid.get_mut((x, y)).unwrap();
    if cell.biome == Biome::Ocean {
        cell.biome = rand::random();
    }

    let (grid_dim_x, grid_dim_y) = grid.dim();
    if depth < max_depth {
        if x < grid_dim_x - 1 { grid_fill(&mut grid, depth + 1, max_depth, (x + 1, y)); }
        if x > 1 { grid_fill(&mut grid, depth + 1, max_depth, (x - 1, y)); }
        if y < grid_dim_y - 1 { grid_fill(&mut grid, depth + 1, max_depth, (x, y + 1)); }
        if y > 1 { grid_fill(&mut grid, depth + 1, max_depth, (x, y - 1)); }
    }
}

impl GameState {
    fn new() -> Result<GameState, String> {
        let quad_data = Buffer::new();
        let quad_instance_data = Buffer::new();
        let quad_vao = Vao::new(quad_data, quad_instance_data);
        quad_data.data(&mut RECT.to_vec(), gl::STATIC_DRAW);
        let mut grid = Array2::from_shape_fn(
            (50, 50),
            |(x, y)| {
                GridCell {
                    biome: Biome::Ocean,
                    unit: None,
                }
            }
        );

        let num_continents = 10;
        let (grid_dim_x, grid_dim_y) = grid.dim();

        let mut rng = rand::thread_rng();
        let rand_x = Uniform::from(0..grid_dim_x);
        let rand_y = Uniform::from(0..grid_dim_y);

        for i in 0..num_continents {
            let x = rand_x.sample(&mut rng);
            let y = rand_y.sample(&mut rng);
            grid_fill(&mut grid, 0, 10, (x, y));
        }

        let mut rect_positions: Vec<_> = grid.indexed_iter().map(|((x, y), grid)| {
            [
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 0.0),
            ]
        }).collect();
        quad_instance_data.data(&mut rect_positions, gl::DYNAMIC_DRAW);

        let cube_data = Buffer::new();
        cube_data.data(&mut CUBE.to_vec(), gl::STATIC_DRAW);
        let cube_instance_data = Buffer::new();
        let cube_vao = Vao::new(cube_data, cube_instance_data);

        Ok(GameState {
            cursor: Default::default(),
            grid,
            solid: Pipeline::new(VERTEX, FRAGMENT)?,

            quad_data,
            quad_instance_data,
            quad_vao,

            cube_data,
            cube_instance_data,
            cube_vao,

            running: true,
            yanked_location: None,
        })
    }
}


fn load(context: &glutin::Context<PossiblyCurrent>) {
    gl::load_with(|ptr| context.get_proc_address(ptr) as *const _);

    let version = unsafe {
        CStr::from_ptr(gl::GetString(gl::VERSION) as *const _).to_str().unwrap()
    };

    println!("Opengl Version: {}", version);
}

pub static RECT: [[f32; 3]; 6] = [
    [1.0, 1.0, 0.0],
    [-1.0, 1.0, 0.0],
    [1.0, -1.0, 0.0],

    [-1.0, -1.0, 0.0],
    [1.0, -1.0, 0.0],
    [-1.0, 1.0, 0.0],
];

pub static CUBE: [[f32; 3]; 36] = [
    [-1.0, -1.0, -1.0],
    [-1.0, -1.0,  1.0],
    [-1.0,  1.0,  1.0],
    [1.0,  1.0, -1.0 ],
    [-1.0, -1.0, -1.0],
    [-1.0,  1.0, -1.0],
    [1.0, -1.0,  1.0 ],
    [-1.0, -1.0, -1.0],
    [1.0, -1.0, -1.0 ],
    [1.0,  1.0, -1.0 ],
    [1.0, -1.0, -1.0 ],
    [-1.0, -1.0, -1.0],
    [-1.0, -1.0, -1.0],
    [-1.0,  1.0,  1.0],
    [-1.0,  1.0, -1.0],
    [1.0, -1.0,  1.0 ],
    [-1.0, -1.0,  1.0],
    [-1.0, -1.0, -1.0],
    [-1.0,  1.0,  1.0],
    [-1.0, -1.0,  1.0],
    [1.0, -1.0,  1.0 ],
    [1.0,  1.0,  1.0 ],
    [1.0, -1.0, -1.0 ],
    [1.0,  1.0, -1.0 ],
    [1.0, -1.0, -1.0 ],
    [1.0,  1.0,  1.0 ],
    [1.0, -1.0,  1.0 ],
    [1.0,  1.0,  1.0 ],
    [1.0,  1.0, -1.0 ],
    [-1.0,  1.0, -1.0],
    [1.0,  1.0,  1.0 ],
    [-1.0,  1.0, -1.0],
    [-1.0,  1.0,  1.0],
    [1.0,  1.0,  1.0 ],
    [-1.0,  1.0,  1.0],
    [1.0, -1.0,  1.0 ],
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
        gl_Position = proj * view * model * vec4(+aWorldPos + aVertOffset, 1.0);
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

    let mut game_state = GameState::new()?;

    let mut world = World::new();
    world.register::<GridPosition>();
    let pos = (0, 0);
    let ent = world.create_entity().with(GridPosition { xy: pos }).build();
    game_state.grid.get_mut(pos).unwrap().unit = Some(ent.id());

    world.insert(game_state);
    world.insert(Camera::new());

    let mut input_state = InputState::new();

    let mut render_system = RenderSystem;

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
                    WindowEvent::KeyboardInput { input, .. } if input.state == ElementState::Pressed => {
                        input_state.event(&mut world, input);
                    },
                    _ => {},
                }
            },
            _ => { },
        };

        render_system.run_now(&world);
        context.swap_buffers().unwrap();

        world.exec(|(game_state): (ReadExpect<crate::GameState>)| {
            if !game_state.running {
                *control_flow = ControlFlow::Exit;
            }
        });
    });

}

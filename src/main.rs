
extern crate glutin;
extern crate cgmath;

mod pipeline;

use cgmath::prelude::*;
use cgmath::*;
use gl::types::*;
use glutin::ContextBuilder;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ PossiblyCurrent, };
use pipeline::*;
use std::ffi::CStr;
use std::mem;

static DEFAULT_GRID_LENGTH: usize = 4;

enum Biome {
    Desert,
    Grassland,
    Hill,
    Mountain,
    Ocean,
    Snow,
}

struct GridCell {
    biome: Biome,
}

struct Grid {
    data: Vec<GridCell>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            data: Vec::with_capacity(width * height),
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> &GridCell {
        &self.data[y * self.width + x]
    }
}

struct GameState {
    grid: Grid,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            grid: Grid::new(DEFAULT_GRID_LENGTH, DEFAULT_GRID_LENGTH),
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

static VERTICES: [f32; 9] = [
     -0.5, -0.5, 0.0,
     0.5, -0.5, 0.0,
     0.0,  0.5, 0.0
];

static VERTEX: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;

    void main() {
        gl_Position = vec4(aPos.xyz, 1.0);
    }
"#;

static FRAGMENT: &str = r#"
    #version 330 core
    out vec4 FragColor;

    void main() {
        FragColor = vec4(1.0, 0.5, 1.0, 1.0);
    }
"#;

struct Renderer {
    solid: Pipeline,
    vao: u32,
    vbo: u32,
}

fn render(renderer: &Renderer) {
    unsafe {
        gl::ClearColor(1.0, 0.5, 0.7, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        renderer.solid.set_use();

        let decomp = Decomposed {
            scale: 1.0,
            rot: Quaternion::new(0.0f32, 0.0, 0.0, 0.0),
            disp: Vector3::new(0.5, -0.5, 0.0),
        };

        let transform: Matrix4<f32> = decomp.into();

        let transform_loc = gl::GetUniformLocation(renderer.solid.0, "transform".as_ptr() as *const i8);
        gl::UniformMatrix4fv(transform_loc, 1, gl::FALSE, transform.as_ptr());


        gl::BindVertexArray(renderer.vao);

        gl::DrawArrays(gl::TRIANGLES, 0, 3);
    }
}

fn main() -> Result<(), String> {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new().with_title("Cim");

    let context = ContextBuilder::new()
        .build_windowed(window_builder, &event_loop)
        .unwrap();

    let context = unsafe { context.make_current().unwrap() };

    load(context.context());
    let solid = Pipeline::new(VERTEX, FRAGMENT)?;

    let gamestate = GameState::new();

    let (vao, vbo) = unsafe {
        let mut vao: u32 = 0;
        gl::GenVertexArrays(1, &mut vao as *mut _);
        let mut vbo: u32 = 0;
        gl::GenBuffers(1, &mut vbo as *mut _);

        gl::EnableVertexAttribArray(vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, mem::size_of_val(&VERTICES) as isize, VERTICES.as_ptr() as *mut _, gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<f32>() as i32, std::ptr::null());

        gl::EnableVertexAttribArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        (vao, vbo)
    };

    let renderer = Renderer {
        solid,
        vao,
        vbo,
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => {
                match event {
                    WindowEvent::Resized(logical_size) => {
                        let dpi_factor = context.window().hidpi_factor();
                        context.resize(logical_size.to_physical(dpi_factor));
                    },
                    WindowEvent::RedrawRequested => {
                        render(&renderer);
                        context.swap_buffers().unwrap();
                    },
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => {},
                }
            },
            _ => { },
        }
    });

}

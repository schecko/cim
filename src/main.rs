
extern crate glutin;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::{ PossiblyCurrent, };
use std::ffi::CStr;
use gl::types::*;
use std::mem;

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
        FragColor = vec4(1.0, 1.0, 1.0, 1.0);
    }
"#;

struct Shader(u32);

unsafe fn compile_shader(source: &str, shader_type: u32) -> Result<Shader, String> {
    let vertex_shader = gl::CreateShader(shader_type);
    let shaders = [
        source,
    ];
    let counts = [
        source.len() as i32,
    ];
    gl::ShaderSource(vertex_shader, 1, shaders.as_ptr() as *const *const i8, counts.as_ptr() as *const _);
    gl::CompileShader(vertex_shader);

    let mut success = 0;
    let mut info_log = [0u8; 512];
    gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success as *mut _);
    if success == 0 {
        gl::GetShaderInfoLog(vertex_shader, info_log.len() as _, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut i8);
        return Err(String::from_utf8_lossy(&info_log[..]).to_string())
    }

    Ok(Shader(vertex_shader))
}

struct Pipeline(u32);

impl Pipeline {
    fn new(vert: &str, frag: &str) -> Result<Pipeline, String> {
        let pipeline = unsafe {
            let vertex_shader = compile_shader(vert, gl::VERTEX_SHADER)?;
            let fragment_shader = compile_shader(frag, gl::FRAGMENT_SHADER)?;

            let pipeline = gl::CreateProgram();
            gl::AttachShader(pipeline, vertex_shader.0);
            gl::AttachShader(pipeline, fragment_shader.0);
            gl::LinkProgram(pipeline);

            let mut success = 0;
            let mut info_log = [0u8; 512];
            gl::GetProgramiv(pipeline, gl::LINK_STATUS, &mut success as *mut _);
            if success == 0 {
                gl::GetProgramInfoLog(pipeline, info_log.len() as _, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut i8);
                return Err(String::from_utf8_lossy(&info_log[..]).to_string())
            }

            gl::DeleteShader(vertex_shader.0);
            gl::DeleteShader(fragment_shader.0);

            pipeline
        };

        Ok(Pipeline(pipeline))
    }
}

struct Renderer {
    pipeline: Pipeline,
    vao: u32,
    vbo: u32,
}


fn render(renderer: &Renderer) {
    unsafe {
        gl::ClearColor(1.0, 0.5, 0.7, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::UseProgram(renderer.pipeline.0);
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
    let pipeline = Pipeline::new(VERTEX, FRAGMENT)?;
    let (vao, vbo) = unsafe {
        let mut vao: u32 = 0;
        gl::GenVertexArrays(1, &mut vao as *mut _);
        let mut vbo: u32 = 0;
        gl::GenBuffers(1, &mut vbo as *mut _);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, mem::size_of_val(&vbo) as isize, VERTICES.as_ptr() as *mut _, gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<f32>() as i32, std::ptr::null());

        gl::EnableVertexAttribArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        (vao, vbo)
    };

    let renderer = Renderer {
        pipeline,
        vao,
        vbo,
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

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

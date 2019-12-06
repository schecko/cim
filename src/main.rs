
extern crate glutin;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::{ PossiblyCurrent, };
use std::ffi::CStr;
use gl::types::*;

fn load(context: &glutin::Context<PossiblyCurrent>) {
    gl::load_with(|ptr| context.get_proc_address(ptr) as *const _);

    let version = unsafe {
        CStr::from_ptr(gl::GetString(gl::VERSION) as *const _).to_str().unwrap()
    };

    println!("Opengl Version: {}", version);
}

fn render(context: &glutin::ContextWrapper<PossiblyCurrent, glutin::window::Window>) {
    unsafe {
        gl::ClearColor(1.0, 0.5, 0.7, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
    context.swap_buffers().unwrap();
}

fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new().with_title("Cim");

    let context = ContextBuilder::new()
        .build_windowed(window_builder, &event_loop)
        .unwrap();

    let context = unsafe { context.make_current().unwrap() };

    load(context.context());

    let mut frame = 0;
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
                        render(&context);
                    },
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => {},
                }
            },
            _ => { },
        }

        frame += 1;
        println!("frame: {}", frame);
    });

}

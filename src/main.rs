#[macro_use] extern crate strum_macros;
extern crate cgmath;
extern crate glutin;
extern crate ndarray;
extern crate rand;
extern crate strum;
extern crate num;
extern crate rusttype;

mod pipeline;
mod renderer;
mod game_state;
mod ogl;
mod input;

use cgmath::*;
use crate::game_state::*;
use crate::input::*;
use crate::ogl::*;
use crate::renderer::*;
use glutin::ContextBuilder;
use glutin::event::{Event, WindowEvent, DeviceEvent, ElementState, };
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ PossiblyCurrent, };
use ndarray::*;
use pipeline::*;
use rusttype::*;
use rusttype::gpu_cache::*;
use std::ffi::CStr;

static VERTEX_TEXT: &str = r#"
    #version 330 core
    in vec2 aPosition;
    in vec2 aTexCoord;
    in vec4 aColor;

    out vec2 vTexCoord;
    out vec4 vColor;

    void main() {
        gl_Position = vec4(aPosition.x, -aPosition.y, 0.0, 1.0);
        vTexCoord = aTexCoord;
        vColor = aColor;
    }
"#;

static FRAGMENT_TEXT: &str = r#"
    #version 330 core
    #extension GL_ARB_explicit_uniform_location : enable

    in vec2 vTexCoord;
    in vec4 vColor;

    out vec4 fColor;

    uniform sampler2D font_cache;

    void main() {
        float a = texture(font_cache, vTexCoord).r;
        fColor = vec4(vColor.rgb, a);
    }
"#;

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
                rot: Quaternion::look_at(Vector3::new(0.0, -1.0, 1.0), Vector3::new(0.0, 1.0, 0.0)),
                disp: Vector3::new(0.0f32, 30.0, -30.0),
            },
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

pub struct World {
    game_state: GameState,
    camera: Camera,
}

#[no_mangle]
extern "system" fn opengl_message_callback(_source: u32, t: u32, _id: u32, _severity: u32, _length: i32, message: *const i8, _user: *mut std::ffi::c_void) {
    unsafe {
        if t == gl::DEBUG_TYPE_ERROR {
            println!(" type: {:x?} message: {}", t, std::ffi::CStr::from_ptr(message).to_string_lossy());
        }
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
    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(Some(opengl_message_callback), std::ptr::null());
    }

    // FONT LOADING
    let font_data = include_bytes!("../arialbd.ttf");
    let font = Font::from_bytes(font_data as &[u8]).unwrap();
    let dpi_factor = context.window().scale_factor();
    let cache_width = (512. * dpi_factor.round()) as u32;
    let cache_height = (512. * dpi_factor.round()) as u32;
    let mut text_cache = Cache::builder()
        .dimensions(cache_width, cache_height)
        .align_4x4(true)
        .build();
    let mut texture: u32 = 0;
    unsafe {
        gl::GenTextures(1, &mut texture as *mut _);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        let data = vec![0x69u8; cache_width as usize * cache_height as usize];
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RED as i32,
            cache_width as i32,
            cache_height as i32,
            0,
            gl::RED,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as _
        );
    }

    let text_pipeline = Pipeline::new(VERTEX_TEXT, FRAGMENT_TEXT)?;
    let text_buffer = Buffer::new();
    let text_vao = Vao::text_new(text_buffer);

    let mut world = World {
        game_state: GameState::new()?,
        camera: Camera::new(),
    };

    let mut input_state = InputState::new();
    let mut renderer = Renderer::new(&world.game_state)?;
    let mut last_frame = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::DeviceEvent { event, .. } => {
                match event {
                    DeviceEvent::ModifiersChanged(state) => {
                        input_state.modifiers = state;
                    },
                    _ => {}
                }
            },
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => {
                match event {
                    WindowEvent::Resized(physical_size) => {
                        context.resize(*physical_size);
                        unsafe { gl::Viewport(0, 0, physical_size.width as _, physical_size.height as _); }
                    },
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } if input.state == ElementState::Pressed => {
                        input_state.event(&mut world, input);
                    },
                    WindowEvent::ReceivedCharacter(c) => {
                        world.game_state.command_text.push(*c);
                    },
                    _ => {
                    },
                }
            },
            Event::RedrawRequested(_) => {
                let frame_start = std::time::Instant::now();
                world.game_state.validate_state();

                if world.game_state.current_player != 0 { // ai player, move their units
                    let player = &mut world.game_state.players[world.game_state.current_player] as *mut Player;
                    (*player).units.iter().for_each(|uid| {
                        let u = rand::random::<usize>();
                        let dirs = [
                            Vector2::new(1, 0),
                            Vector2::new(-1, 0),
                            Vector2::new(0, 1),
                            Vector2::new(0, -1),
                        ];

                        match world.game_state.get_unit_mut(uid) {
                            Some(unit) => {
                                let dest = unit.loc + dirs[u % dirs.len()];
                                if (world.game_state.grid_contains(dest)) {
                                }
                            },
                            None => return,
                        }
                    });
                }

                world.game_state.check_turn_complete(false);

                let dt = (frame_start - last_frame).as_secs_f64();
                last_frame = frame_start;
                let fps = 1. / dt;

                let text_verts = { // FONT RENDERING
                    let mut glyphs: Vec<PositionedGlyph<'_>> = Vec::new();
                    // TODO figure out why scale_factor blows up on linux
                    //let factor = context.window().scale_factor().round();
                    let screen_size = context.window().inner_size();
                    let scale = Scale::uniform(100.0);
                    let v_metrics = font.v_metrics(scale);

                    { // bottom left text
                        let mut caret = point(screen_size.width as f32 * -1., screen_size.height as f32);
                        let command_text = format!("{} {}", input_state.mode.to_string(), world.game_state.command_text);

                        for c in command_text.chars() {
                            let base_glyph = font.glyph(c);
                            let glyph = base_glyph.scaled(scale).positioned(caret);
                            caret.x += glyph.unpositioned().h_metrics().advance_width;
                            glyphs.push(glyph);
                        }
                    }

                    { // top left text
                        let mut caret = point(screen_size.width as f32 * -1., -(screen_size.height as f32) + v_metrics.ascent);
                        let fps_text = format!("{:2.1} fps", fps);

                        for c in fps_text.chars() {
                            let base_glyph = font.glyph(c);
                            let glyph = base_glyph.scaled(scale).positioned(caret);
                            caret.x += glyph.unpositioned().h_metrics().advance_width;
                            glyphs.push(glyph);
                        }
                    }

                    { // top right text
                        let mut caret = point(screen_size.width as f32 * 1., -(screen_size.height as f32) + v_metrics.ascent);
                        let current_player = if world.game_state.current_player == 0 {
                            "U0".to_owned()
                        } else {
                            format!("AI{}", world.game_state.current_player)
                        };
                        let turn_text = format!("player {} turn {}", current_player, world.game_state.turn);

                        for c in turn_text.chars().rev() {
                            let base_glyph = font.glyph(c);
                            let scaled = base_glyph.scaled(scale);
                            caret.x -= scaled.h_metrics().advance_width;
                            let glyph = scaled.positioned(caret);
                            glyphs.push(glyph);
                        }
                    }

                    glyphs.iter().for_each(|glyph| {
                        text_cache.queue_glyph(0, glyph.clone());
                    });

                    text_cache.cache_queued(|rect, data| {
                        assert!(rect.width() as usize * rect.height() as usize == data.len());
                        unsafe {
                            gl::BindTexture(gl::TEXTURE_2D, texture);
                            assert!(gl::GetError() == 0);
                            gl::TexSubImage2D(
                                gl::TEXTURE_2D,
                                0,
                                rect.min.x as i32,
                                rect.min.y as i32,
                                rect.width() as i32 - 1,
                                rect.height() as i32,
                                gl::RED,
                                gl::UNSIGNED_BYTE,
                                data.as_ptr() as _
                            );
                            assert!(gl::GetError() == 0);
                        }
                    }).expect("Failed to render glyph");

                    text_pipeline.set_use();
                    let font_tex_location = text_pipeline.get_uniform_location("font_cache");
                    assert!(font_tex_location >= 0);
                    unsafe {
                        gl::Uniform1i(font_tex_location, 0);
                        assert!(gl::GetError() == 0);
                    }

                    let mut text_verts: Vec<[[f32; 8]; 6]> = glyphs
                        .iter()
                        .filter_map(|glyph| {
                            if let Ok(data) = text_cache.rect_for(0, glyph) {
                                data
                            } else {
                                None
                            }
                        })
                        .map(|(uv, pix_loc)| {
                            let window_size = context.window().inner_size();
                            let width = window_size.width as f32;
                            let height = window_size.height as f32;
                            let loc = Rect {
                                min: point(pix_loc.min.x as f32 / width * 1., pix_loc.min.y as f32 / height * 1.),
                                max: point(pix_loc.max.x as f32 / width * 1., pix_loc.max.y as f32 / height * 1.),
                            };

                            [
                                [
                                    // pos
                                    loc.min.x, loc.max.y, // bottom right
                                    // uv
                                    uv.min.x, uv.max.y,
                                    // color
                                    0.0, 0.0, 0.0, 1.0
                                ],
                                [
                                    loc.min.x, loc.min.y,
                                    uv.min.x, uv.min.y,
                                    0.0, 0.0, 0.0, 1.0
                                ],
                                [
                                    loc.max.x, loc.min.y,
                                    uv.max.x, uv.min.y,
                                    0.0, 0.0, 0.0, 1.0
                                ],
                                [
                                    loc.max.x, loc.min.y,
                                    uv.max.x, uv.min.y,
                                    0.0, 0.0, 0.0, 1.0
                                ],
                                [
                                    loc.max.x, loc.max.y,
                                    uv.max.x, uv.max.y,
                                    0.0, 0.0, 0.0, 1.0
                                ],
                                [
                                    loc.min.x, loc.max.y,
                                    uv.min.x, uv.max.y,
                                    0.0, 0.0, 0.0, 1.0
                                ],
                            ]
                        })
                        .collect();
                        text_buffer.data(&mut text_verts, gl::DYNAMIC_DRAW);

                        text_verts
                };

                let current_turn = world.game_state.turn;
                unsafe {
                    let game_state: *mut GameState = &mut world.game_state as *mut _;
                    world.game_state.structures
                        .iter_mut()
                        .for_each(|entity| {
                            if let Some(structure) = &mut entity.data {
                                let cell = (*game_state).get_grid(structure.loc);
                                if structure.next_unit_ready <= current_turn && cell.unit.is_none() {
                                    let unit = Unit::from(structure as & _);
                                    structure.next_unit_ready = current_turn + 5;
                                    (*game_state).add_unit(unit);
                                }
                            }
                        });
                }

                renderer.render(&mut world.game_state, &mut world.camera);

                unsafe {
                    text_pipeline.set_use();
                    gl::BindVertexArray(text_vao.0);
                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::BindTexture(gl::TEXTURE_2D, texture);
                    gl::Disable(gl::DEPTH_TEST);
                    gl::Enable(gl::BLEND);
                    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                    gl::DrawArrays(gl::TRIANGLES, 0, text_verts.len() as i32 * 6);
                }

                context.swap_buffers().unwrap();

                if !world.game_state.running {
                    *control_flow = ControlFlow::Exit;
                }
                context.window().request_redraw();
            },
            Event::RedrawEventsCleared => {
                context.window().request_redraw();
            },
            _ => { },
        };
    });

}

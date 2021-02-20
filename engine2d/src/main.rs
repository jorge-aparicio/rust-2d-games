use objects::*;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use std::time::Instant;

const DT: f64 = 1.0 / 60.0;
const DEPTH: usize = 4;
const WIDTH: usize = 240;
const HEIGHT: usize = 360;

// We'll make our Color type an RGBA8888 pixel.

mod collision;
mod generation;
//mod music;
mod objects;

// pixels gives us an rgba8888 framebuffer
fn clear(fb: &mut [u8], c: Color) {
    // Four bytes per pixel; chunks_exact_mut gives an iterator over 4-element slices.
    // So this way we can use copy_from_slice to copy our color slice into px very quickly.
    for px in fb.chunks_exact_mut(4) {
        px.copy_from_slice(&c);
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let mut player = Rect {
        pos: Vec2::new(30.0, HEIGHT as f32 / 2.0 - 10.0), // idk what this looks like
        size: Vec2::new(20.0, 20.0),
    };
    let mut generate = generation::Obstacles {
        obstacles: vec![
            (
                MovingRect {
                    pos: Vec2::new(50.0, 0.0),
                    vel: Vec2::new(0.02, 0.0),
                    size: Vec2::new(80.0, 80.0),
                },
                MovingRect {
                    pos: Vec2::new(50.0, HEIGHT as f32 - 80.0),
                    vel: Vec2::new(0.02, 0.0),
                    size: Vec2::new(80.0, 80.0),
                },
            ),
            (
                MovingRect {
                    pos: Vec2::new(50.0, 0.0),
                    vel: Vec2::new(0.02, 0.0),
                    size: Vec2::new(80.0, 80.0),
                },
                MovingRect {
                    pos: Vec2::new(100.0, HEIGHT as f32 - 80.0),
                    vel: Vec2::new(0.02, 0.0),
                    size: Vec2::new(60.0, 40.0),
                },
            ),
        ],
        frequency_values: Vec::new(),
    };
    let mut obstacles = Vec::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("flappy bird")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
    };
    let colors = [
        [255, 0, 0, 255],
        [255, 255, 0, 255],
        [0, 255, 0, 255],
        [0, 255, 255, 255],
        [0, 0, 255, 255],
        [255, 0, 255, 255],
    ];
    let start = Instant::now();
    let mut available_time = 0.0;
    let mut since = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let fb = pixels.get_frame();
            clear(fb, [0, 0, 0, 255]);
            obstacles.push(generate.generate_obstacles());

            filled_rect(fb,
                (player.pos.x.trunc() as i32, player.pos.y.trunc() as i32),
                (player.size.x.trunc() as i32, player.size.y.trunc() as i32),
                colors[5]);
            
            // draw obstacles
            for (top,bottom)  in obstacles.iter() {
                filled_rect(
                    fb,
                    (
                        top.pos.x.trunc() as i32,
                        top.pos.y.trunc() as i32,
                    ),
                    (
                        top.size.x.trunc() as i32,
                        top.size.y.trunc() as i32,
                    ),
                    colors[0],
                );
                filled_rect(
                    fb,
                    (
                        bottom.pos.x.trunc() as i32,
                        bottom.pos.y.trunc() as i32,
                    ),
                    (
                        bottom.size.x.trunc() as i32,
                        bottom.size.y.trunc() as i32,
                    ),
                    colors[0],
                );
            }

            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            available_time += since.elapsed().as_secs_f64();
        }
        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }
        }

        while available_time >= DT {
            available_time -= DT;
            // move all obstacles
            for (top,bottom)  in obstacles.iter_mut() {
                top.pos.x += top.vel.x;
                top.pos.y += top.vel.y;

                bottom.pos.x += bottom.vel.x;
                bottom.pos.y += bottom.vel.y;
                
            }
            window.request_redraw();
        }
    });
}

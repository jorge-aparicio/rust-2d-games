use objects::*;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use rodio::Source;
use std::fs::File;
use std::io::BufReader;

use std::time::{Duration, Instant};

const DT: f64 = 1.0 / 60.0;
const DEPTH: usize = 4;
const WIDTH: usize = 240;
const HEIGHT: usize = 360;

// We'll make our Color type an RGBA8888 pixel.

mod collision;
mod generation;
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
    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    let file1 = File::open("birdcoo.mp3").unwrap();
    let file2 = File::open("birdflap.mp3").unwrap();
    let file3 = File::open("city-quiet.mp3").unwrap();
    let source1 = rodio::Decoder::new(BufReader::new(file1)).unwrap();
    let source2 = rodio::Decoder::new(BufReader::new(file2)).unwrap();
    let source3 = rodio::Decoder::new(BufReader::new(file3)).unwrap();

    let _source1 = source1
        .take_duration(Duration::from_secs(9))
        .repeat_infinite();
    let _source2 = source2
        .take_duration(Duration::from_secs(4))
        .repeat_infinite();
    let _source3 = source3
        .take_duration(Duration::from_secs(31))
        .repeat_infinite();

    stream_handle.play_raw(_source1.convert_samples());
    stream_handle.play_raw(_source2.convert_samples());
    stream_handle.play_raw(_source3.convert_samples());

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let mut player = MovingRect {
        pos: Vec2::new(30.0, HEIGHT as f32 / 2.0 - 10.0), // idk what this looks like
        size: Vec2::new(20.0, 20.0),
        vel: Vec2::new(0.0, 0.0),
    };
    let mut generate = generation::Obstacles {
        obstacles: vec![
            (
                MovingRect {
                    pos: Vec2::new(220.0, 0.0),
                    vel: Vec2::new(0.2, 0.0),
                    size: Vec2::new(20.0, 80.0),
                },
                MovingRect {
                    pos: Vec2::new(220.0, HEIGHT as f32 - 80.0),
                    vel: Vec2::new(0.2, 0.0),
                    size: Vec2::new(20.0, 80.0),
                },
            ),
            (
                MovingRect {
                    pos: Vec2::new(220.0, 0.0),
                    vel: Vec2::new(0.2, 0.0),
                    size: Vec2::new(20.0, 100.0),
                },
                MovingRect {
                    pos: Vec2::new(220.0, HEIGHT as f32 - 80.0),
                    vel: Vec2::new(0.2, 0.0),
                    size: Vec2::new(20.0, 60.0),
                },
            ),
        ],
        frequency_values: vec![1, 3],
    };
    let mut obstacles: Vec<MovingRect> = Vec::new();
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
    let mut last_added_rect = Instant::now();
    let mut available_time = 0.0;
    let mut since = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let fb = pixels.get_frame();
            clear(fb, [0, 0, 0, 255]);

            filled_rect(
                fb,
                (player.pos.x.trunc() as i32, player.pos.y.trunc() as i32),
                (player.size.x.trunc() as i32, player.size.y.trunc() as i32),
                colors[5],
            );

            // draw obstacles
            
            for obstacle in obstacles.iter() {
                filled_rect(
                    fb,
                    (obstacle.pos.x.trunc() as i32, obstacle.pos.y.trunc() as i32),
                    (obstacle.size.x.trunc() as i32, obstacle.size.y.trunc() as i32),
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
            if input.key_pressed(VirtualKeyCode::Space) {
                player.vel.y = 1.0;
            }

            available_time -= DT;
            // update acceleration for bird
            player.vel.y -= 0.04;

            // update position
            player.pos.x -= player.vel.x;
            player.pos.y -= player.vel.y;
            for obstacle in obstacles.iter_mut() {
                obstacle.pos.x -= obstacle.vel.x;
                obstacle.pos.y += obstacle.vel.y;
            }

            let contacts = collision::gather_contacts(&player, &obstacles);
            for contact in contacts.iter() {
                use collision::{Contact, ContactID};
                match contact.get_ids() {
                    (ContactID::Player, ContactID::Obstacle) => {
                        player.pos = Vec2::new(30.0, HEIGHT as f32 / 2.0 - 10.0);
                        player.vel = Vec2::new(0.0, 0.0);
                        obstacles.clear();
                        last_added_rect = Instant::now();
                    }
                    _ => {}
                }
            }

            if since.duration_since(last_added_rect) >= Duration::from_secs(3) {
                let (top, bottom) = generate.generate_obstacles();
                obstacles.push(top);
                obstacles.push(bottom);
                last_added_rect = Instant::now();
            }
        }

        since = Instant::now();
        window.request_redraw();
    });
}

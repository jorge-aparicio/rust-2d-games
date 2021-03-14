use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;

use pixels::{Pixels, SurfaceTexture};
use rodio::Source;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper; //, PlayError};

mod animation;
mod collision;
mod generation;
mod input;
mod objects;
mod screen;
mod sprite;
mod text;
mod texture;

use objects::*;

use std::time::{Duration, Instant};

const DT: f64 = 1.0 / 60.0;
const WIDTH: usize = 240;
const HEIGHT: usize = 360;
const DEPTH: usize = 4;

struct GameState {
    player: MovingRect,
    obstacles: Vec<Rect>,
    obstacle_data: Vec<ObstacleData>,
    move_vel: f32,
    score: u32,
    time_between: u32,
    text_info: text::TextInfo,
}

struct ObstacleData {
    passed: bool,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum ActionID {
    Flap,
}

fn main() {
    let (_stream, _stream_handle) = rodio::OutputStream::try_default().unwrap();

    let file1 = File::open("../content/birdcoo.mp3").unwrap();
    let file2 = File::open("../content/birdflap.mp3").unwrap();
    let file3 = File::open("../content/city-quiet.mp3").unwrap();
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

    // let _ = stream_handle.play_raw(_source1.convert_samples());
    // let _ = stream_handle.play_raw(_source2.convert_samples());
    // let _ = stream_handle.play_raw(_source3.convert_samples());

    let event_loop = EventLoop::new();
    let mut input_events = WinitInputHelper::new();
    let generate = generation::Obstacles {
        obstacles: vec![(80, 120), (160, 130), (70, 230)],
        frequency_values: vec![1, 1, 1],
    };

    let mut state = GameState {
        player: MovingRect::new(
            30.0,
            HEIGHT as f32 / 2.0 - 10.0,
            20.0,
            20.0,
            Vec2::new(0.0, 0.0),
        ),
        obstacles: Vec::new(),
        obstacle_data: Vec::new(),
        score: 0,
        move_vel: 1.0,
        time_between: 3000,
        text_info: {
            use std::path::Path;
            let image = Rc::new(texture::Texture::with_file(Path::new(
                "../content/ascii.png",
            )));
            let char_size = 16.0;
            let info = [
                (' ', Rect::new(0.0, 0.0, char_size, char_size)),
                ('s', Rect::new(48.0, 80.0, char_size, char_size)),
                ('c', Rect::new(48.0, 64.0, char_size, char_size)),
                ('o', Rect::new(240.0, 64.0, char_size, char_size)),
                ('r', Rect::new(32.0, 80.0, char_size, char_size)),
                ('e', Rect::new(80.0, 64.0, char_size, char_size)),
                (':', Rect::new(160.0, 16.0, char_size, char_size)),
                ('0', Rect::new(0.0, 16.0, char_size, char_size)),
                ('1', Rect::new(16.0, 16.0, char_size, char_size)),
                ('2', Rect::new(32.0, 16.0, char_size, char_size)),
                ('3', Rect::new(48.0, 16.0, char_size, char_size)),
                ('4', Rect::new(64.0, 16.0, char_size, char_size)),
                ('5', Rect::new(80.0, 16.0, char_size, char_size)),
                ('6', Rect::new(96.0, 16.0, char_size, char_size)),
                ('7', Rect::new(112.0, 16.0, char_size, char_size)),
                ('8', Rect::new(128.0, 16.0, char_size, char_size)),
                ('9', Rect::new(144.0, 16.0, char_size, char_size)),
            ];
            text::TextInfo::new(&image, &info)
        },
    };
    let mut input = input::Input::new();
    input.add_key_to_map(ActionID::Flap, VirtualKeyCode::Space);

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

    let mut last_added_rect = Instant::now();
    let mut available_time = 0.0;
    let mut since = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let mut screen = screen::Screen::wrap(pixels.get_frame(), WIDTH, HEIGHT, DEPTH);
            screen.clear([0, 0, 0, 255]);

            screen.rect(state.player.as_rect(), [255, 0, 255, 255]);

            // draw state.obstacles
            for obstacle in state.obstacles.iter() {
                screen.rect(*obstacle, [255, 0, 0, 255]);
            }

            use crate::text::DrawTextExt;
            screen.draw_text_at_pos(
                format!("score: {}", state.score),
                Vec2::new(0.0, 0.0),
                &state.text_info,
            );

            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            available_time += since.elapsed().as_secs_f64();
        }
        // Handle input_events events
        if input_events.update(&event) {
            // Close events
            if input_events.key_pressed(VirtualKeyCode::Escape) || input_events.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input_events.window_resized() {
                pixels.resize(size.width, size.height);
            }
        }

        while available_time >= DT {
            since = Instant::now();
            available_time -= DT;

            input.update(&input_events);
            if input.is_pressed(ActionID::Flap) {
                state.player.vel.y = 2.0;
            }

            // update velocity for bird
            state.player.vel.y -= 0.04;

            // update position
            state.player.x -= state.player.vel.x;
            state.player.y -= state.player.vel.y;
            for obstacle in state.obstacles.iter_mut() {
                obstacle.x -= state.move_vel;
            }

            let contacts = collision::gather_contacts(&state.player, &state.obstacles);
            for contact in contacts.iter() {
                use collision::ContactID;
                if let (ContactID::Player, ContactID::Obstacle) = contact.get_ids() {
                    // TODO: have a function that resets the game state??
                    state.player.x = 30.0;
                    state.player.y = HEIGHT as f32 / 2.0 - 10.0;
                    state.player.vel = Vec2::new(0.0, 0.0);
                    state.obstacles.clear();
                    state.obstacle_data.clear();
                    state.time_between = 3000;
                    state.move_vel = 1.0;
                    state.score = 0;
                    last_added_rect = Instant::now();
                    since = Instant::now();
                }
            }

            if state.obstacles.len() >= 2 && state.obstacles[0].x + state.obstacles[0].w <= 0.0 {
                // remove the first two state.obstacles
                state.obstacles.remove(0);
                state.obstacles.remove(0);
                state.obstacle_data.remove(0);
                state.obstacle_data.remove(0);
            }

            if since.duration_since(last_added_rect)
                >= Duration::from_millis(state.time_between as u64)
            {
                let (top, bottom) = generate.generate_obstacles();
                state
                    .obstacles
                    .push(Rect::new(WIDTH as f32, 0.0, 20.0, top as f32));
                state.obstacles.push(Rect::new(
                    WIDTH as f32,
                    HEIGHT as f32 - bottom as f32,
                    20.0,
                    bottom as f32,
                ));
                state.obstacle_data.push(ObstacleData { passed: false });
                state.obstacle_data.push(ObstacleData { passed: false });
                last_added_rect = Instant::now();
                since = Instant::now();
            }

            for (i, (obst, data)) in state
                .obstacles
                .iter_mut()
                .zip(state.obstacle_data.iter_mut())
                .enumerate()
            {
                if state.player.x > obst.x && !data.passed {
                    data.passed = true;
                    if i % 2 == 0 {
                        state.score += 1;
                        if state.move_vel < 3.0 {
                            state.move_vel *= 1.1;
                        }
                        if state.obstacles.len() >= 4
                            && state.obstacles[state.obstacles.len() - 1].x
                                - state.obstacles[state.obstacles.len() - 3].x
                                > state.obstacles[state.obstacles.len() - 3].w * 2.0
                        {
                            state.time_between = (state.time_between - 200).max(800);
                        }
                        break;
                    }
                }
            }
        }

        window.request_redraw();
    });
}

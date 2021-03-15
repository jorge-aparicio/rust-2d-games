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
const CHAR_SIZE: f32 = 16.0;


#[derive(Debug)]
enum Mode {
    Title,
    Play,
    EndGame,
}

struct GameState {
    player: MovingRect,
    obstacles: Vec<Rect>,
    obstacle_data: Vec<ObstacleData>,
    move_vel: f32,
    score: u32,
    time_between: u32,
    text_info: text::TextInfo,
    mode: Mode,
}

struct ObstacleData {
    passed: bool,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum ActionID {
    Flap,
}

fn main() {
    let info = [
                            (' ', Rect::new(0.0, 0.0, CHAR_SIZE, CHAR_SIZE)),
                            ('!', Rect::new(16.0, 0.0, CHAR_SIZE, CHAR_SIZE)),
                            ('a', Rect::new(16.0, 64.0, CHAR_SIZE, CHAR_SIZE)),
                            ('b', Rect::new(32.0, 64.0, CHAR_SIZE, CHAR_SIZE)),
                            ('c', Rect::new(48.0, 64.0, CHAR_SIZE, CHAR_SIZE)),
                            ('d', Rect::new(64.0, 64.0, CHAR_SIZE, CHAR_SIZE)),
                            ('e', Rect::new(80.0, 64.0, CHAR_SIZE, CHAR_SIZE)),
                            ('f', Rect::new(96.0, 64.0, CHAR_SIZE, CHAR_SIZE)),
                            ('g', Rect::new(112.0, 64.0, CHAR_SIZE, CHAR_SIZE)),
                            ('h', Rect::new(128.0, 64.0, CHAR_SIZE, CHAR_SIZE)),
                            ('i', Rect::new(144.0, 64.0, CHAR_SIZE, CHAR_SIZE)),
                            ('j', Rect::new(160.0, 64.0, CHAR_SIZE, CHAR_SIZE)),
                            ('k', Rect::new(176.0, 64.0, CHAR_SIZE, CHAR_SIZE)),
                            ('l', Rect::new(192.0, 64.0, CHAR_SIZE, CHAR_SIZE)),
                            ('m', Rect::new(208.0, 64.0, CHAR_SIZE, CHAR_SIZE)),
                            ('n', Rect::new(224.0, 64.0, CHAR_SIZE, CHAR_SIZE)),
                            ('o', Rect::new(240.0, 64.0, CHAR_SIZE, CHAR_SIZE)),
                            ('p', Rect::new(0.0, 80.0, CHAR_SIZE, CHAR_SIZE)),
                            ('q', Rect::new(16.0, 80.0, CHAR_SIZE, CHAR_SIZE)),
                            ('r', Rect::new(32.0, 80.0, CHAR_SIZE, CHAR_SIZE)),
                            ('s', Rect::new(48.0, 80.0, CHAR_SIZE, CHAR_SIZE)),
                            ('t', Rect::new(64.0, 80.0, CHAR_SIZE, CHAR_SIZE)),
                            ('u', Rect::new(80.0, 80.0, CHAR_SIZE, CHAR_SIZE)),
                            ('v', Rect::new(96.0, 80.0, CHAR_SIZE, CHAR_SIZE)),
                            ('w', Rect::new(112.0, 80.0, CHAR_SIZE, CHAR_SIZE)),
                            ('x', Rect::new(128.0, 80.0, CHAR_SIZE, CHAR_SIZE)),
                            ('y', Rect::new(144.0, 80.0, CHAR_SIZE, CHAR_SIZE)),
                            ('z', Rect::new(160.0, 80.0, CHAR_SIZE, CHAR_SIZE)),
                            (':', Rect::new(160.0, 16.0, CHAR_SIZE, CHAR_SIZE)),
                            ('0', Rect::new(0.0, 16.0, CHAR_SIZE, CHAR_SIZE)),
                            ('1', Rect::new(16.0, 16.0, CHAR_SIZE, CHAR_SIZE)),
                            ('2', Rect::new(32.0, 16.0, CHAR_SIZE, CHAR_SIZE)),
                            ('3', Rect::new(48.0, 16.0, CHAR_SIZE, CHAR_SIZE)),
                            ('4', Rect::new(64.0, 16.0, CHAR_SIZE, CHAR_SIZE)),
                            ('5', Rect::new(80.0, 16.0, CHAR_SIZE, CHAR_SIZE)),
                            ('6', Rect::new(96.0, 16.0, CHAR_SIZE, CHAR_SIZE)),
                            ('7', Rect::new(112.0, 16.0, CHAR_SIZE, CHAR_SIZE)),
                            ('8', Rect::new(128.0, 16.0, CHAR_SIZE, CHAR_SIZE)),
                            ('9', Rect::new(144.0, 16.0, CHAR_SIZE, CHAR_SIZE)),
                        ];
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
            text::TextInfo::new(&image, &info)
        },
        mode: Mode::Title,
    };

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
        match state.mode {
            Mode::Title => {
                // Draw the current frame
                if let Event::RedrawRequested(_) = event {
                    let mut screen = screen::Screen::wrap(pixels.get_frame(), WIDTH, HEIGHT, DEPTH);
                    screen.clear([135, 206, 250, 150]);
                    //screen.rect(state.player.as_rect(), [255, 0, 255, 255]);
                    //screen.rect(state.player.as_rect(), [255, 0, 255, 255]);

                    // draw state.obstacles
                    //for obstacle in state.obstacles.iter() {
                    //  screen.rect(*obstacle, [255, 0, 0, 255]);
                    //}

                    use crate::text::DrawTextExt;
                    screen.draw_text_at_pos(
                        format!("score: {}", state.score),
                        Vec2::new(0.0, 0.0),
                        &state.text_info,
                    );
                   
                    

                    screen.draw_text_at_pos(format!("flappy pigeon"), Vec2::new(20.0, 60.0), {
                        use std::path::Path;
                        let image = Rc::new(texture::Texture::with_file(Path::new(
                            "../content/ascii.png",
                        )));
                        
                        &text::TextInfo::new(&image, &info)
                    });

                    screen.draw_text_at_pos(format!("press enter"), Vec2::new(40.0, 240.0), {
                        use std::path::Path;
                        let image = Rc::new(texture::Texture::with_file(Path::new(
                            "../content/ascii.png",
                        )));
                        
                        &text::TextInfo::new(&image, &info)
                    });
                    screen.draw_text_at_pos(format!("to start"), Vec2::new(65.0, 260.0), {
                        use std::path::Path;
                        let image = Rc::new(texture::Texture::with_file(Path::new(
                            "../content/ascii.png",
                        )));
                        
                        &text::TextInfo::new(&image, &info)
                    });

                    if pixels.render().is_err() {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }

                    //available_time += since.elapsed().as_secs_f64();
                }
                // Handle input_events events
                if input_events.update(&event) {
                    // Close events
                    if input_events.key_pressed(VirtualKeyCode::Escape) || input_events.quit() {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    if input_events.key_pressed(VirtualKeyCode::Return) {
                        state.mode = Mode::Play;
                        state.player.x = 30.0;
                        state.player.y = HEIGHT as f32 / 2.0 - 10.0;
                        state.player.vel = Vec2::new(0.0, 0.0);
                        //state.obstacles.clear();
                        //state.obstacle_data.clear();
                        state.time_between = 3000;
                        state.move_vel = 1.0;
                        //state.score = 0;
                        last_added_rect = Instant::now();
                        since = Instant::now();
                        //return;
                    }
                    // Resize the window
                    if let Some(size) = input_events.window_resized() {
                        pixels.resize(size.width, size.height);
                    }
                }
            }
            Mode::Play => {
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
                            state.mode = Mode::EndGame
                        }
                    }

                    if state.obstacles.len() >= 2
                        && state.obstacles[0].x + state.obstacles[0].w <= 0.0
                    {
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
                
            }
            Mode::EndGame => {
                if let Event::RedrawRequested(_) = event {
                    let mut screen = screen::Screen::wrap(pixels.get_frame(), WIDTH, HEIGHT, DEPTH);
                    screen.clear([200, 0, 0, 150]);
                    //screen.rect(state.player.as_rect(), [255, 0, 255, 255]);
                    //screen.rect(state.player.as_rect(), [255, 0, 255, 255]);

                    // draw state.obstacles
                    //for obstacle in state.obstacles.iter() {
                    //  screen.rect(*obstacle, [255, 0, 0, 255]);
                    //}

                    use crate::text::DrawTextExt;
                    screen.draw_text_at_pos(
                        format!("score: {}", state.score),
                        Vec2::new(0.0, 0.0),
                        &state.text_info,
                    );
                    

                    screen.draw_text_at_pos(format!("game over!!!"), Vec2::new(20.0, 60.0), {
                        use std::path::Path;
                        let image = Rc::new(texture::Texture::with_file(Path::new(
                            "../content/ascii.png",
                        )));
                        
                        &text::TextInfo::new(&image, &info)
                    });

                    screen.draw_text_at_pos(format!("press enter"), Vec2::new(40.0, 240.0), {
                        use std::path::Path;
                        let image = Rc::new(texture::Texture::with_file(Path::new(
                            "../content/ascii.png",
                        )));
                        
                        &text::TextInfo::new(&image, &info)
                    });
                    screen.draw_text_at_pos(format!("to try again"), Vec2::new(30.0, 260.0), {
                        use std::path::Path;
                        let image = Rc::new(texture::Texture::with_file(Path::new(
                            "../content/ascii.png",
                        )));
                        
                        &text::TextInfo::new(&image, &info)
                    });

                    if pixels.render().is_err() {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }
                    if input_events.update(&event) {
                        // Close events
                        if input_events.key_pressed(VirtualKeyCode::Escape) || input_events.quit() {
                            *control_flow = ControlFlow::Exit;
                            return;
                        }
                        if input_events.key_pressed(VirtualKeyCode::Return) {
                            state.mode = Mode::Play;
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
                            //return;
                        }
                        // Resize the window
                        if let Some(size) = input_events.window_resized() {
                            pixels.resize(size.width, size.height);
                            
                        }
                    }
            }
        }
        window.request_redraw();

    });
}

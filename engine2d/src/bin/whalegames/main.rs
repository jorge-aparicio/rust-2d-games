#![allow(unused)]
use std::cmp;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use std::rc::Rc;

use pixels::{Pixels, SurfaceTexture};
use rodio::Source;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper; //, PlayError};
use substring::Substring;

use engine2d::{
    objects::*,
    screen::Screen,
    sprite::{DrawSpriteExt, Sprite},
    text::*,
    texture::Texture,
    animation::{Animation, AnimationData}
};

mod storyparser;

const DT: f64 = 1.0 / 60.0;
const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const DEPTH: usize = 4;
pub const CHAR_SIZE: f32 = 16.0;
const BOX_COLOR: Color = [255, 255, 255, 255];
const BOX_X: f32 = WIDTH as f32 / 10.0;
const BOX_Y: f32 = 6.0 * HEIGHT as f32 / 11.0;
const BOX_WIDTH: f32 = 8.0 * WIDTH as f32 / 10.0;
const BOX_HEIGHT: f32 = 4.0 * HEIGHT as f32 / 10.0;

#[derive(Debug)]
enum Mode {
    Title,
    Read,
    Respond,
    EndGame,
}


use storyparser::*;
struct GameState {
    scene_map: HashMap<String, Scene>,
    current_scene: Scene,
    box_read: bool,
    message_index: usize,
    box_text_index: usize,
    response_index: usize,
    text_info: TextInfo,
    mode: Mode,
}

impl GameState {
    pub fn reset_read_info(&mut self){
        self.message_index = 0;
        self.box_text_index = 0;
        self.response_index = 0;
        self.box_read = false;
    }

    pub fn reset_game(&mut self){
        self.reset_read_info();
        self.current_scene =  self.scene_map.get("intro").unwrap().clone();
        self.mode = Mode::Title;

    }
}

mod textinfo;

fn main() {
    let text_box: Rect = Rect::new(BOX_X, BOX_Y, BOX_WIDTH, BOX_HEIGHT);

    let story = parse_story().unwrap();
    let title = story.story_name.clone();
    let mut scene_map: HashMap<String,Scene> = HashMap::new();
    let mut sprites: HashMap<String, Sprite> = HashMap::new();
    use std::path::Path;
    story.scenes.iter().for_each(|s| {
        scene_map.insert(s.scene_name.clone(), s.scene.clone());
        let texture = Texture::with_file(Path::new(&format!("content/fishsprites/{}.png", s.scene.name)));
        let width = texture.width as f32;
        let height = texture.height as f32;
        let animation = Animation::new(&Rc::new(AnimationData {
            frames: vec![(Rect::new(0.0, 0.0, width, height), 1)],
            looping: false,
        }));
        sprites.insert(s.scene.name.clone(), Sprite::new(
            &Rc::new(texture),
            animation,
            Vec2::new((WIDTH as f32 - width) / 2.0, 200.0 - height)
        ));

   });

    let current_scene = scene_map.get("intro").unwrap();
    let mut state = GameState {
        // add tree struct that will represent game text and options. empty until text parser implemented
        scene_map : scene_map.clone(),
        current_scene: current_scene.clone(),
        box_read: false,
        message_index: 0,
        box_text_index: 0,
        response_index: 0,
        // position in tree
        //ending_score: 0,
        // ending determiner
        text_info: {
            use std::path::Path;
            let image = Rc::new(Texture::with_file(Path::new(
                "content/ascii-dark.png",
            )));
            TextInfo::new(&image, &textinfo::info())
        },
        mode: Mode::Title,
    };

    let (_stream, _stream_handle) = rodio::OutputStream::try_default().unwrap();

    /* let file1 = File::open("content/birdcoo.mp3").unwrap();
    let file2 = File::open("content/birdflap.mp3").unwrap();
    let file3 = File::open("content/city-quiet.mp3").unwrap();
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
    */

    let event_loop = EventLoop::new();
    let mut input_events = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Finding Nemo: The After Story")
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

    event_loop.run(move |event, _, control_flow| {
        match state.mode {
            Mode::Title => {
                // Draw the current frame
                if let Event::RedrawRequested(_) = event {
                    let mut screen = Screen::wrap(pixels.get_frame(), WIDTH, HEIGHT, DEPTH);
                    screen.clear([135, 206, 250, 150]);

                    screen.draw_text_at_pos(
                        "Finding Nemo: The After Story",
                        Vec2::new(500.0, 160.0),
                        &state.text_info,
                    );
                    screen.draw_text_at_pos(
                        "press enter to start.",
                        Vec2::new(460.0, 440.0),
                        &state.text_info,
                    );
                    screen.draw_text_at_pos("", Vec2::new(65.0, 260.0), &state.text_info);

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
                        state.mode = Mode::Read;
                        //return;
                    }

                    // Resize the window
                    if let Some(size) = input_events.window_resized() {
                        pixels.resize(size.width, size.height);
                    }
                }
            }
            Mode::Read => {
                // Draw the current frame
                let mut end_line_index: usize;
                let mut start_line_index: usize = state.box_text_index;
                if let Event::RedrawRequested(_) = event {
                    let mut screen = Screen::wrap(pixels.get_frame(), WIDTH, HEIGHT, DEPTH);
                    screen.clear([135, 206, 250, 150]);

                    //render text box
                    screen.rect(text_box, BOX_COLOR);
                    screen.rect_lines(text_box, [0, 0, 0, 0]);

                    // draw sprite
                    if let Some(sprite) = sprites.get(&state.current_scene.name) {
                        screen.draw_sprite(sprite);
                    }

                    // render text in box as many characters that will fit per line for now
                    for i in 1..(BOX_HEIGHT / (CHAR_SIZE + 1.0)) as usize {
                        end_line_index = cmp::min(
                            start_line_index + ((BOX_WIDTH / CHAR_SIZE) - 1.0) as usize,
                            state.current_scene.message.len(),
                        );
                        screen.draw_text_at_pos(
                            state
                                .current_scene
                                .message
                                .substring(start_line_index, end_line_index),
                            Vec2::new(BOX_X + 10.0, BOX_Y + ((CHAR_SIZE) * i as f32)),
                            &state.text_info,
                        );
                        start_line_index = end_line_index;
                    }
                    state.message_index = start_line_index;

                    //println!(" start index: {}",start_line_index);
                    if pixels.render().is_err() {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }


                // Handle input_events events
                if input_events.update(&event) {
                    // Close events

                    if input_events.key_pressed(VirtualKeyCode::Escape) || input_events.quit() {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }

                    if input_events.key_pressed(VirtualKeyCode::Space) || input_events.quit() {
                        state.box_text_index = state.message_index;
                        if state.current_scene.responses.len() > 0 && state.current_scene.responses[0].response != ""  {
                            // if player has read all text and has option to give response switch to response mode
                            println!(
                                " responses not empty, start index: {}, message length: {}",
                                start_line_index,
                                state.current_scene.message.len()
                            );
                            if state.message_index >= state.current_scene.message.len() - 1 {
                                state.mode = Mode::Respond;
                                state.box_read = false;
                                state.box_text_index = 0;
                                state.message_index = 0;
                            }
                        } else {
                            // if player reached end of tree and no final response available switch to game over
                            if state.current_scene.responses.is_empty() {
                                state.mode = Mode::EndGame;
                            } else {
                                // if no response option available go forward in story
                                state.current_scene = state.scene_map.get(&*(state.current_scene.responses[0].goto)).unwrap().clone();
                                state.reset_read_info();

                            }
                        }

                        return;
                    }

                    // Resize the window
                    if let Some(size) = input_events.window_resized() {
                        pixels.resize(size.width, size.height);
                    }
                }
            }

            Mode::Respond => {
                // Draw the current frame

                if let Event::RedrawRequested(_) = event {
                    let mut screen = Screen::wrap(pixels.get_frame(), WIDTH, HEIGHT, DEPTH);
                    // render background
                    screen.clear([135, 206, 250, 150]);

                    //render text box
                    screen.rect(text_box, BOX_COLOR);
                    screen.rect_lines(text_box, [0, 0, 0, 0]);

                    

                    // vec of response y values for pointer to know location
                    let mut ypos_vec: Vec<(f32, f32)> = vec![];

                    //render responses
                    for (i, resp_map) in state.current_scene.responses.iter().enumerate() {
                        // render text in box as many characters that will fit per line for now
                        let mut end_line_index: usize;
                        let mut start_line_index: usize = 0;

                        for j in 1 ..(BOX_HEIGHT / (CHAR_SIZE)) as usize {
                            end_line_index = cmp::min(
                                start_line_index
                                    + (((BOX_WIDTH - 3.0 * BOX_WIDTH / 64.0) / CHAR_SIZE) - 1.0)
                                        as usize,
                                resp_map.response.len(),
                            );
                            screen.draw_text_at_pos(
                                resp_map.response.substring(start_line_index, end_line_index),
                                Vec2::new(
                                    BOX_X + 3.0 * BOX_WIDTH / 64.0,
                                    BOX_Y + ((CHAR_SIZE * 2.0) * i as f32) + (CHAR_SIZE * j as f32),
                                ),
                                &state.text_info,
                            );
                            start_line_index = end_line_index;
                        }
                        ypos_vec.push((
                            BOX_Y + ((CHAR_SIZE * 2.0) * i as f32),
                            (resp_map.response.len() as f32
                                / (((BOX_WIDTH - 3.0 * BOX_WIDTH / 64.0) / CHAR_SIZE) - 1.0)),
                        ));
                    }

                    // response pointer
                    let pointer = Rect {
                        x: BOX_X + 1.0 * BOX_WIDTH / 64.0,
                        y: {
                            let (init_y, i) = ypos_vec[state.response_index];
                            println!("{}", i);
                            init_y + (i) * CHAR_SIZE
                        },
                        h: 8.0,
                        w: 8.0,
                    };
                    screen.rect(pointer, [255, 0, 0, 255]);

                    //TODO  character, character name?

                    if pixels.render().is_err() {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }

                //TODO update position in tree

                // wait for player input

                // Handle input_events events
                if input_events.update(&event) {
                    // Close events
                    if input_events.key_pressed(VirtualKeyCode::Escape) || input_events.quit() {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }

                    if input_events.key_pressed(VirtualKeyCode::Down) || input_events.quit() {
                        //TODO change response option
                        if state.response_index < state.current_scene.responses.len() - 1 {
                            state.response_index += 1;
                        } else {
                            state.response_index = 0;
                        }
                        return;
                    }

                    if input_events.key_pressed(VirtualKeyCode::Up) || input_events.quit() {
                        if state.response_index > 0 {
                            state.response_index -= 1;
                        } else {
                            state.response_index = state.current_scene.responses.len() - 1;
                        }
                        return;
                    }

                    if input_events.key_pressed(VirtualKeyCode::Space) || input_events.quit() {
                        //move to next value in tree based on response.
                        if state.current_scene.responses.is_empty() {
                            state.mode = Mode::EndGame;
                        } else {
                            state.current_scene = state.scene_map.get(&*(state.current_scene.responses[0].goto)).unwrap().clone();
                            state.reset_read_info();
                            state.mode = Mode::Read;
                        }

                        return;
                    }

                    // Resize the window
                    if let Some(size) = input_events.window_resized() {
                        pixels.resize(size.width, size.height);
                    }
                }
            }

            Mode::EndGame => {
                if let Event::RedrawRequested(_) = event {
                    let mut screen = Screen::wrap(pixels.get_frame(), WIDTH, HEIGHT, DEPTH);
                    screen.clear([200, 0, 0, 150]);

                    screen.draw_text_at_pos("the end", Vec2::new(400.0, 60.0), &state.text_info);

                    screen.draw_text_at_pos(
                        "press enter to return to title screen",
                        Vec2::new(400.0, 240.0),
                        &state.text_info,
                    );
                    screen.draw_text_at_pos(
                        "or escape to exit",
                        Vec2::new(300.0, 260.0),
                        &state.text_info,
                    );

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
                        // reset game mode to title, state values to default
                        state.reset_game();
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

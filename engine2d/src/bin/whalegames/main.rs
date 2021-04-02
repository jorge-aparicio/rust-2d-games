#![allow(unused)]
use std::cmp;
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
use std::collections::HashMap;


mod storyparser;
use crate::text::DrawTextExt;
use engine2d::{objects::*, screen, text, texture};
use substring::Substring;

const DT: f64 = 1.0 / 60.0;
const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const DEPTH: usize = 4;
const CHAR_SIZE: f32 = 16.0;
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
    text_info: text::TextInfo,
    mode: Mode,
}

impl GameState {
    pub fn reset_read_info(&mut self){
        self.message_index = 0;
        self.box_text_index = 0;
        self.response_index = 0;
        self.box_read = false;
    }
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
    

    let text_box: Rect = Rect::new(BOX_X, BOX_Y, BOX_WIDTH, BOX_HEIGHT);

    let scene_map = parse_story().unwrap();
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
            let image = Rc::new(texture::Texture::with_file(Path::new(
                "content/ascii-dark.png",
            )));
            text::TextInfo::new(&image, &info)
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
            .with_title("finding nemo: the after story")
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
                    let mut screen = screen::Screen::wrap(pixels.get_frame(), WIDTH, HEIGHT, DEPTH);
                    screen.clear([135, 206, 250, 150]);

                    screen.draw_text_at_pos(
                        "the whale games",
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
                    let mut screen = screen::Screen::wrap(pixels.get_frame(), WIDTH, HEIGHT, DEPTH);
                    screen.clear([135, 206, 250, 150]);

                    //render text box
                    screen.rect(text_box, BOX_COLOR);
                    screen.rect_lines(text_box, [0, 0, 0, 0]);

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

                // TODO update position in tree

                // Handle input_events events
                if input_events.update(&event) {
                    // Close events

                    if input_events.key_pressed(VirtualKeyCode::Escape) || input_events.quit() {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }

                    if input_events.key_pressed(VirtualKeyCode::Space) || input_events.quit() {
                        state.box_text_index = state.message_index;
                        if state.current_scene.responses.len() > 1 {
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
                    let mut screen = screen::Screen::wrap(pixels.get_frame(), WIDTH, HEIGHT, DEPTH);
                    // render background
                    screen.clear([135, 206, 250, 150]);

                    //render text box
                    screen.rect(text_box, BOX_COLOR);
                    screen.rect_lines(text_box, [0, 0, 0, 0]);

                    // render text in box as many characters that will fit per line for now
                    let mut end_line_index: usize;
                    let mut start_line_index: usize = 0;
                    let mut respmess_numlines = 0;
                    for i in 1..(BOX_HEIGHT / (CHAR_SIZE + 1.0)) as usize {
                        end_line_index = cmp::min(
                            start_line_index + ((BOX_WIDTH / CHAR_SIZE) - 1.0) as usize,
                            state.current_scene.response_message.len(),
                        );
                        screen.draw_text_at_pos(
                            state
                                .current_scene
                                .response_message
                                .substring(start_line_index, end_line_index),
                            Vec2::new(BOX_X + 10.0, BOX_Y + ((CHAR_SIZE) * i as f32)),
                            &state.text_info,
                        );
                        start_line_index = end_line_index;
                        respmess_numlines+= 1;
                    }

                    // vec of response y values for pointer to know location
                    let mut ypos_vec: Vec<(f32, f32)> = vec![];

                    //render responses
                    for (i, resp_map) in state.current_scene.responses.iter().enumerate() {
                        // render text in box as many characters that will fit per line for now
                        let mut end_line_index: usize;
                        let mut start_line_index: usize = 0;

                        for j in (1 + respmess_numlines)..(BOX_HEIGHT / (CHAR_SIZE)) as usize {
                            end_line_index = cmp::min(
                                start_line_index
                                    + (((BOX_WIDTH - 3.0 * BOX_WIDTH / 64.0) / CHAR_SIZE) - 1.0)
                                        as usize,
                                resp_map.response.len() - 1,
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
                    let mut screen = screen::Screen::wrap(pixels.get_frame(), WIDTH, HEIGHT, DEPTH);
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
                        // reset gamemode tree position
                        state.mode = Mode::Title;
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

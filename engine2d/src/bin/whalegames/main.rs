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

mod generation;

use engine2d::{animation, collision, input, objects::*, screen, sprite, text, texture};

use std::time::{Duration, Instant};

const DT: f64 = 1.0 / 60.0;
const WIDTH: usize = 480;
const HEIGHT: usize = 360;
const DEPTH: usize = 4;
const CHAR_SIZE: f32 = 16.0;

#[derive(Debug)]
enum Mode {
    Title,
    Read,
    Respond,
    EndGame,
}

struct GameState {
    // add tree struct that will represent game text and options

    // position in tree

    // ending determiner
    text_info: text::TextInfo,
    mode: Mode,
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
        // add tree struct that will represent game text and options

        // position in tree

        // ending determiner
        text_info: {
            use std::path::Path;
            let image = Rc::new(texture::Texture::with_file(Path::new("content/ascii.png")));
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
    

    let mut input = input::Input::new();

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

                    use crate::text::DrawTextExt;
                    
                    screen.draw_text_at_pos(
                        "The Whale Games",
                        Vec2::new(20.0, 60.0),
                        &state.text_info,
                    );
                    screen.draw_text_at_pos(
                        "Press ENTER to start.",
                        Vec2::new(40.0, 240.0),
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
                        state.mode = Mode::Play;
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
                if let Event::RedrawRequested(_) = event {
                    let mut screen = screen::Screen::wrap(pixels.get_frame(), WIDTH, HEIGHT, DEPTH);
                    screen.clear([0, 0, 0, 255]);

                    //TODO render background, text box, text, character

                    
                    if pixels.render().is_err() {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }

                }

                // TODO update position in tree

                // wait for player input
                loop {
                    // Handle input_events events
                if input_events.update(&event) {
                    // Close events
                    if input_events.key_pressed(VirtualKeyCode::Escape) || input_events.quit() {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }

                    if input_events.key_pressed(VirtualKeyCode::Space) || input_events.quit() {
                        // TODO based on next value in tree remain in read mode or switch to respond mode
                        return;
                    }
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
                    screen.clear([0, 0, 0, 255]);

                    //TODO render background, text box, character, response pointer

                    
                    if pixels.render().is_err() {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }

                }

                //TODO update position in tree

                // wait for player input
                loop {
                    // Handle input_events events
                if input_events.update(&event) {
                    // Close events
                    if input_events.key_pressed(VirtualKeyCode::Escape) || input_events.quit() {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }

                    if input_events.key_pressed(VirtualKeyCode::Down) || input_events.quit() {
                        //TODO change response option
                        return;
                    }

                    if input_events.key_pressed(VirtualKeyCode::Up) || input_events.quit() {
                        //TODO change response option
                        return;
                    }

                    if input_events.key_pressed(VirtualKeyCode::Space) || input_events.quit() {
                        //TODO based on next value in tree remain in read mode or switch to respond mode
                        return;
                    }
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

                    use crate::text::DrawTextExt;

                    screen.draw_text_at_pos(
                        "The End",
                        Vec2::new(20.0, 60.0),
                        &state.text_info,
                    );

                    screen.draw_text_at_pos(
                        "press enter to return to title screen",
                        Vec2::new(40.0, 240.0),
                        &state.text_info,
                    );
                    screen.draw_text_at_pos(
                        "or escape to exit",
                        Vec2::new(30.0, 260.0),
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

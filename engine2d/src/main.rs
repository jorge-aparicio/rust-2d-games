use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const DEPTH: usize = 4;
const WIDTH: usize = 240;
const HEIGHT: usize = 360;

// We'll make our Color type an RGBA8888 pixel.
type Color = [u8; DEPTH];

// pixels gives us an rgba8888 framebuffer
fn clear(fb: &mut [u8], c: Color) {
    // Four bytes per pixel; chunks_exact_mut gives an iterator over 4-element slices.
    // So this way we can use copy_from_slice to copy our color slice into px very quickly.
    for px in fb.chunks_exact_mut(4) {
        px.copy_from_slice(&c);
    }
}

#[allow(dead_code)]
fn hline(fb: &mut [u8], x0: usize, x1: usize, y: usize, c: Color) {
    assert!(y < HEIGHT);
    assert!(x0 <= x1);
    assert!(x1 < WIDTH);
    for p in fb[(y * WIDTH * 4 + x0 * 4)..(y * WIDTH * 4 + x1 * 4)].chunks_exact_mut(4) {
        p.copy_from_slice(&c);
    }
}

#[allow(dead_code)]
fn line(fb: &mut [u8], (x0, y0): (i32, i32), (x1, y1): (i32, i32), col: Color) {
    let mut x = x0 as i64;
    let mut y = y0 as i64;
    let x0 = x0 as i64;
    let y0 = y0 as i64;
    let x1 = x1 as i64;
    let y1 = y1 as i64;
    let dx = (x1 - x0).abs();
    let sx: i64 = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy: i64 = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    while x != x1 || y != y1 {
        if (x >= 0 && x < WIDTH as i64) && (y >= 0 && y < HEIGHT as i64) {
            fb[(y as usize * WIDTH * DEPTH + x as usize * DEPTH)
                ..(y as usize * WIDTH * DEPTH + (x as usize + 1) * DEPTH)]
                .copy_from_slice(&col);
        }
        let e2 = 2 * err;
        if dy <= e2 {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

struct MovingRect {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

fn filled_rect(fb: &mut [u8], (x, y): (i32, i32), (w, h): (i32, i32), col: Color) {
    (y..y + h).for_each(|y| {
        line(fb, (x, y), (x + w, y), col);
    });
}

fn filled_circle(fb: &mut [u8], (x, y): (i32, i32), r: u64, col: Color) {
    for i in x - r as i32..x + r as i32 {
        for j in y - r as i32..y + r as i32 {
            if dist((i, j), (x, y)) < r as f32
                && (x >= 0 && x < WIDTH as i32)
                && (y >= 0 && y < HEIGHT as i32)
            {
                fb[WIDTH * DEPTH * j as usize + i as usize * DEPTH
                    ..WIDTH * DEPTH * j as usize + (i + 1) as usize * DEPTH]
                    .copy_from_slice(&col);
            }
        }
    }
}

fn dist((x0, y0): (i32, i32), (x1, y1): (i32, i32)) -> f32 {
    let dx = (x0 - x1) as f32;
    let dy = (y0 - y1) as f32;
    (dx * dx + dy * dy).sqrt()
}

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let mut moving_rect = MovingRect {
        x: 50.0,
        y: 50.0,
        vx: 0.02,
        vy: 0.0,
    };
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

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            clear(pixels.get_frame(), [0, 0, 0, 255]);
            filled_rect(
                pixels.get_frame(),
                (moving_rect.x.trunc() as i32, moving_rect.y.trunc() as i32),
                (80, 80),
                colors[0],
            );

            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
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
        moving_rect.x += moving_rect.vx;
        moving_rect.y += moving_rect.vy;
        window.request_redraw();
    });
}


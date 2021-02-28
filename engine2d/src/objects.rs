#![allow(dead_code)]
const DEPTH: usize = 4;
const WIDTH: usize = 240;
const HEIGHT: usize = 360;

pub type Color = [u8; DEPTH];
pub fn hline(fb: &mut [u8], x0: usize, x1: usize, y: usize, c: Color) {
    assert!(y < HEIGHT);
    assert!(x0 <= x1);
    assert!(x1 < WIDTH);
    for p in fb[(y * WIDTH * 4 + x0 * 4)..(y * WIDTH * 4 + x1 * 4)].chunks_exact_mut(4) {
        p.copy_from_slice(&c);
    }
}

#[allow(dead_code)]
pub fn line(fb: &mut [u8], (x0, y0): (i32, i32), (x1, y1): (i32, i32), col: Color) {
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

#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone)]
pub struct MovingRect {
    pub pos: Vec2,
    pub vel: Vec2,
    pub size: Vec2,
}

pub struct Rect {
    pub pos: Vec2,
    pub size: Vec2,
}

pub fn filled_rect(fb: &mut [u8], (x, y): (i32, i32), (w, h): (i32, i32), col: Color) {
    (y..y + h).for_each(|y| {
        line(fb, (x, y), (x + w, y), col);
    });
}

pub fn filled_circle(fb: &mut [u8], (x, y): (i32, i32), r: u64, col: Color) {
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

pub fn dist((x0, y0): (i32, i32), (x1, y1): (i32, i32)) -> f32 {
    let dx = (x0 - x1) as f32;
    let dy = (y0 - y1) as f32;
    (dx * dx + dy * dy).sqrt()
}

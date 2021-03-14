#![allow(unused)]
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::objects::{Rect, Vec2};
use crate::texture::Texture;

pub struct TextInfo {
    pub info: BTreeMap<char, Rect>,
    image: Rc<Texture>,
}

impl TextInfo {
    pub fn new(image: &Rc<Texture>, char_info: &[(char, Rect)]) -> Self {
        let mut text_info = TextInfo {
            info: BTreeMap::new(),
            image: Rc::clone(image),
        };
        for (character, rect) in char_info.iter() {
            text_info.info.insert(*character, *rect);
        }
        text_info
    }
}

pub trait DrawTextExt {
    fn draw_string_at_pos(&mut self, string: String, pos: Vec2, font: &Rc<TextInfo>);
}

use crate::screen::Screen;
impl<'fb> DrawTextExt for Screen<'fb> {
    fn draw_string_at_pos(&mut self, string: String, pos: Vec2, font: &Rc<TextInfo>) {
        // starting positions
        let mut x = pos.x as f32;
        let y = pos.y as f32;
        for ch in string.chars() {
            if let Some(rect) = font.info.get(&ch) {
                self.bitblt(&font.image, *rect, Vec2::new(x, y));
                x += rect.x;
            }
        }
    }
}

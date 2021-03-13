#![allow(unused)]
use std::collections::BTreeMap;

use crate::objects::{Rect, Vec2};

pub struct TextInfo {
    pub info: BTreeMap<char, Rect>,
}

impl TextInfo {
    pub fn new(char_info: &[(char, Rect)]) -> Self {
        let mut text_info = TextInfo {
            info: BTreeMap::new(),
        };
        for (character, rect) in char_info.iter() {
            text_info.info.insert(*character, *rect);
        }
        text_info
    }

    pub fn draw_string_at_pos(string: String, pos: Vec2) {}
}

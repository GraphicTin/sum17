

use macroquad::prelude::*;

pub struct Number {
    // ID/Data Component
    pub value: u8,
    // Transform Component
    pub center_offset: Vec2,
    // Interaction Component
    pub is_dragging: bool,
    pub drag_offset: Vec2,
}

impl Number {
    /// Creates a new `Number` object.
    pub fn new(value: u8, center_offset: Vec2) -> Self {
        Self {
            value,
            center_offset,
            is_dragging: false,
            drag_offset: Vec2::ZERO,
        }
    }

    pub fn draw(&self, font: &Font, center: &Vec2) {
        let text = self.value.to_string();
        let dims = measure_text(&text, Some(font), 48, 1.0);
        
        draw_text_ex(
            &text,
            self.center_offset.x - (dims.width / 2.0) + center.x,
            self.center_offset.y + (dims.offset_y / 2.0) + center.y,
            TextParams {
                font: Some(font),
                font_size: 48,
                ..Default::default()
            },
        );
    }
}
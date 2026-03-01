use crate::types::color::Color;

#[derive(Debug, Clone)]
pub struct TextStyle {
    pub font_size: u16,
    pub fg_color: Color,
    pub bg_color: Color,
}

impl TextStyle {
    pub fn new(font_size: u16, color: Color, background_color: Color) -> Self {
        TextStyle {
            font_size,
            fg_color: color,
            bg_color: background_color,
        }
    }
}

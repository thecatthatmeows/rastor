use crate::types::color::Color;

pub struct Border {
    pub width: f32,
    pub color: Color,
    pub style: BorderStyle
}

pub enum BorderStyle {
    Solid,
    Dashed { dash_length: f32, gap_length: f32 },
    Dotted,
    None
}
use crate::{shapes::{Shape, line::Line}, types::{color::Color, vec2::Vec2}};

pub enum BorderStyle {
    Solid,
    Dashed { dash_length: f32, gap_length: f32 },
    Dotted,
    None
}

pub struct Border {
    pub width: f32,
    pub color: Color,
    pub style: BorderStyle
}

impl Border {
    pub fn new(width: f32, color: Color, style: BorderStyle) -> Self {
        Self { width, color, style }
    }

    pub fn draw(&self) {
        // Implement drawing logic based on the border style
        match self.style {
            BorderStyle::Solid => {
                // clockwise, starting from top-left corner
                let rect_corners = [
                    Vec2::new(0.0, 0.0), // top left
                    Vec2::new(1.0, 0.0), // top right
                    Vec2::new(1.0, 1.0), // bottom right
                    Vec2::new(0.0, 1.0), // bottom left
                ];
                for i in 0..rect_corners.len() {
                    let rect_corner = rect_corners[i];
                    let mut next_rect_corner =  if i >= rect_corners.len() - 1 {
                        rect_corners[0]
                    } else {
                        rect_corners[i + 1]
                    };
                    Line::new(rect_corner, next_rect_corner, self.color).draw();
                }
            },
            BorderStyle::Dashed { dash_length, gap_length } => {
                // Draw dashed border using dash_length and gap_length
            },
            BorderStyle::Dotted => {
                // Draw dotted border
            },
            BorderStyle::None => {
                // No border to draw
            }
        }
    }
}
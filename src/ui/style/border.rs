use crate::{X_SCALE, shapes::{Shape, line::Line}, types::{color::Color, pos2::Pos2, vec2::Vec2}};

pub enum BorderStyle {
    Solid,
    Dashed { dash_length: f32, gap_length: f32 },
    Dotted,
    None
}

pub struct Border {
    pub pos: Pos2,
    pub width: f32,
    pub color: Color,
    pub style: BorderStyle
}

impl Border {
    pub fn new(pos: Pos2, width: f32, color: Color, style: BorderStyle) -> Self {
        Self {
            pos: pos - Vec2::new(0.0, width / 2.0), // adjust position to account for border width
            // pos,
            width,
            color,
            style
        }
    }

    pub fn draw(&self) {
        // Implement drawing logic based on the border style
        match self.style {
            BorderStyle::Solid => {
                // we're reimplementing the drawing logic again since the triangle drawing here wont work well
                // cuz.. ehmm... you wouldnt want a diagonal line in the middle out of nowhere, would you?
                // clockwise, starting from top-left corner
                let rect_corners = [
                    self.pos + Vec2::new(0.0, 0.0) * self.width, // top left
                    self.pos + Vec2::new(1.0 * X_SCALE, 0.0) * self.width, // top right
                    self.pos + Vec2::new(1.0 * X_SCALE, 1.0) * self.width, // bottom right
                    self.pos + Vec2::new(0.0, 1.0) * self.width, // bottom left
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

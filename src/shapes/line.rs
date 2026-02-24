use std::io::stdout;

use crate::{
    shapes::{Orientation, Shape, pixel::{Pixel, flush_pixels}}, types::{pos2::Pos2, vec2::Vec2}
};
use crossterm::style::Color;
use crossterm::terminal;

pub struct Line {
    pub pos1: Pos2,
    pub pos2: Pos2,
    pub color: Color,
    pub z_index: i32,
}

impl Line {
    pub fn new(pos1: impl Into<Pos2>, pos2: impl Into<Pos2>, color: Color) -> Self {
        Self {
            pos1: pos1.into(),
            pos2: pos2.into(),
            color,
            z_index: 0,
        }
    }
}

/// Simple Clone impl for Line: duplicate the visible state.
impl Clone for Line {
    fn clone(&self) -> Self {
        Self {
            pos1: self.pos1,
            pos2: self.pos2,
            color: self.color,
            z_index: self.z_index,
        }
    }
}

impl Shape for Line {
    fn draw(&mut self) {
        // For compatibility, draw() will rasterize into a local pixel buffer and
        // then flush via the centralized helper in `shapes::mod.rs`.
        let (term_width, term_height) = terminal::size().unwrap();
        let mut pixels: Vec<Pixel> = Vec::with_capacity(1024);
        self.rasterize(&mut pixels, (term_width, term_height));
        let mut out = stdout().lock();
        flush_pixels(&mut out, &mut pixels);
    }

    fn rasterize(&self, out: &mut Vec<Pixel>, term_size: (u16, u16)) {
        let term_width = term_size.0 as i32;
        let term_height = term_size.1 as i32;

        let pos1: Vec2<f32> = self.pos1.into();
        let pos2: Vec2<f32> = self.pos2.into();

        let x0 = pos1.x as i32;
        let y0 = pos1.y as i32;
        let x1 = pos2.x as i32;
        let y1 = pos2.y as i32;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx: i32 = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut x = x0;
        let mut y = y0;

        loop {
            if x >= 0 && x < term_width && y >= 0 && y < term_height {
                out.push(Pixel::new(x as u16, y as u16, '█', self.color, self.z_index));
            }

            if x == x1 && y == y1 {
                break;
            }
            let e2 = err * 2;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    fn update(&mut self) {
        // Lines do not have dynamic geometry to update here by default.
    }

    /// Set this line's position by moving its midpoint to `pos`.
    /// Both endpoints are translated by the same delta so the line's length
    /// and orientation are preserved.
    fn set_pos(&mut self, pos: Pos2) {
        let current_mid: Vec2<f32> = self.pos().into();
        let new_mid: Vec2<f32> = pos.into();
        let delta = new_mid - current_mid;

        let p1: Vec2<f32> = self.pos1.into();
        let p2: Vec2<f32> = self.pos2.into();
        self.pos1 = (p1 + delta).into();
        self.pos2 = (p2 + delta).into();
    }

    fn set_orientation(&mut self, orientation: Orientation) {
        let pos1: Vec2<f32> = self.pos1.into();
        let pos2: Vec2<f32> = self.pos2.into();
        // Rotate the line endpoints around its midpoint to match the requested orientation.
        // Compute current angle and desired angle, rotate by the delta.
        let desired = orientation.to_f32();
        let current = (pos2.y - pos1.y).atan2(pos2.x - pos1.x);
        let delta = desired - current;

        let mid: Vec2<f32> = self.pos().into();
        let pos1: Vec2<f32> = self.pos1.into();
        let pos2: Vec2<f32> = self.pos2.into();
        // translate to origin, rotate, translate back
        let p1 = (pos1 - mid).rotate(delta) + mid;
        let p2 = (pos2 - mid).rotate(delta) + mid;
        self.pos1 = p1.into();
        self.pos2 = p2.into();
    }

    fn orientation(&self) -> Orientation {
        let pos1: Vec2<f32> = self.pos1.into();
        let pos2: Vec2<f32> = self.pos2.into();
        let dx = pos2.x - pos1.x;
        let dy = pos2.y - pos1.y;
        let angle = dy.atan2(dx);
        Orientation::Custom(angle)
    }

    fn z_index(&self) -> i32 {
        self.z_index
    }

    fn pos(&self) -> Pos2 {
            // midpoint of the line
            let pos1: Vec2<f32> = self.pos1.into();
            let pos2: Vec2<f32> = self.pos2.into();
            let mid = Vec2 {
                x: (pos1.x + pos2.x) / 2.0,
                y: (pos1.y + pos2.y) / 2.0,
            };
            mid.into()
        }

    fn box_clone(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }

    fn collides_with(&self, other: &dyn Shape) -> bool {
        // Basic collision: check if other's position is inside the line's bounding box.
        let other_p: Vec2<f32> = other.pos().into();

        let pos1: Vec2<f32> = self.pos1.into();
        let pos2: Vec2<f32> = self.pos2.into();
        // min max so that we check both ends
        let min_x = f32::min(pos1.x, pos2.x);
        let max_x = f32::max(pos1.x, pos2.x);
        let min_y = f32::min(pos1.y, pos2.y);
        let max_y = f32::max(pos1.y, pos2.y);

        other_p.x >= min_x && other_p.x <= max_x && other_p.y >= min_y && other_p.y <= max_y
    }
}

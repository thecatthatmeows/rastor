use std::io::{StdoutLock, Write, stdout};

use crate::{
    buffer::{FRAME_BUFFER, FrameBuffer},
    types::vec2::Vec2,
    shapes::{Orientation, Shape},
};
use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, SetForegroundColor},
    terminal,
};

pub struct Line {
    pub pos1: Vec2<f32>,
    pub pos2: Vec2<f32>,
    pub color: Color,
    pub z_index: i32,
    stdout: StdoutLock<'static>,
}

impl Line {
    pub fn new(pos1: impl Into<Vec2<f32>>, pos2: impl Into<Vec2<f32>>, color: Color) -> Self {
        let (term_width, term_height) = terminal::size().unwrap();
        Self {
            pos1: pos1.into(),
            pos2: pos2.into(),
            color,
            z_index: 0,
            stdout: stdout().lock(),
        }
    }
}

/// Manual Clone impl: StdoutLock isn't Clone, so create a fresh lock for the clone.
/// This mirrors how other shapes create their stdout locks and keeps clone semantics
/// consistent by duplicating the visible state (pos1, pos2, color, z_index) while acquiring
/// a new stdout lock for use in the cloned instance.
impl Clone for Line {
    fn clone(&self) -> Self {
        Self {
            pos1: self.pos1,
            pos2: self.pos2,
            color: self.color,
            z_index: self.z_index,
            stdout: stdout().lock(),
        }
    }
}

impl Shape for Line {
    fn draw(&mut self) {
        let (term_width, term_height) = terminal::size().unwrap();
        let term_width = term_width as i32;
        let term_height = term_height as i32;

        let x0 = self.pos1.x as i32;
        let y0 = self.pos1.y as i32;
        let x1 = self.pos2.x as i32;
        let y1 = self.pos2.y as i32;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx: i32 = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut x = x0;
        let mut y = y0;

        let mut buf = Vec::with_capacity(1024);
        // FRAME_BUFFER.set_color(Color::Green, &mut self.stdout);
        loop {
            if x >= 0 && x < term_width && y >= 0 && y < term_height {
                buf.push((x, y));
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

        // for (x, y) in &buf {
        //     FRAME_BUFFER.set_pixel(*x as usize, *y as usize, '█');
        //     FRAME_BUFFER.move_to(*x as usize, *y as usize, &mut self.stdout);
        // }

        // FRAME_BUFFER.render(&mut self.stdout);

        for (x, y) in &buf {
            queue!(
                self.stdout,
                MoveTo(*x as u16, *y as u16),
                // SetForegroundColor(self.color),
                Print("█")
            )
            .unwrap();
        }
        // self.stdout.flush().unwrap();
    }

    fn update(&mut self) {
        // Lines do not have dynamic geometry to update here by default.
    }

    fn set_orientation(&mut self, orientation: Orientation) {
        // Rotate the line endpoints around its midpoint to match the requested orientation.
        // Compute current angle and desired angle, rotate by the delta.
        let desired = orientation.to_f32();
        let current = (self.pos2.y - self.pos1.y).atan2(self.pos2.x - self.pos1.x);
        let delta = desired - current;

        let mid = self.pos();
        // translate to origin, rotate, translate back
        let p1 = (self.pos1 - mid).rotate(delta) + mid;
        let p2 = (self.pos2 - mid).rotate(delta) + mid;
        self.pos1 = p1;
        self.pos2 = p2;
    }

    fn orientation(&self) -> Orientation {
        let dx = self.pos2.x - self.pos1.x;
        let dy = self.pos2.y - self.pos1.y;
        let angle = dy.atan2(dx);
        Orientation::Custom(angle)
    }

    fn z_index(&self) -> i32 {
        self.z_index
    }

    fn pos(&self) -> Vec2<f32> {
        // midpoint of the line
        Vec2::new((self.pos1.x + self.pos2.x) / 2.0, (self.pos1.y + self.pos2.y) / 2.0)
    }

    fn box_clone(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }

    fn collides_with(&self, other: &dyn Shape) -> bool {
        // Basic collision: check if other's position is inside the line's bounding box.
        let other_p = other.pos();

        // min max so that we check both ends
        let min_x = f32::min(self.pos1.x, self.pos2.x);
        let max_x = f32::max(self.pos1.x, self.pos2.x);
        let min_y = f32::min(self.pos1.y, self.pos2.y);
        let max_y = f32::max(self.pos1.y, self.pos2.y);

        other_p.x >= min_x && other_p.x <= max_x && other_p.y >= min_y && other_p.y <= max_y
    }
}

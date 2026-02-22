use std::io::{StdoutLock, Write, stdout};

use crate::{
    buffer::{FRAME_BUFFER, FrameBuffer},
    types::vec2::Vec2,
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

    pub fn draw(&mut self) {
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

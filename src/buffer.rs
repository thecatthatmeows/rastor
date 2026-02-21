use std::sync::atomic::{AtomicU8, Ordering};

use crate::utils::get_terminal_size;
use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, SetForegroundColor},
};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref FRAME_BUFFER: FrameBuffer = FrameBuffer::new(
        get_terminal_size().unwrap().x as usize,
        get_terminal_size().unwrap().y as usize
    );
}

/// The `FrameBuffer` struct was made to render text-based graphics *faster* than just telling the
/// terminal to move the cursor and print each character individually.
#[deprecated(
    since = "0.1.0",
    note = "Na bro, we sticking to manually telling the terminal what to do"
)]
pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub data: Vec<AtomicU8>,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let data = (0..width * height)
            .map(|_| AtomicU8::new(' ' as u8))
            .collect();
        FrameBuffer {
            width,
            height,
            data,
        }
    }

    pub fn set_pixel(&self, x: usize, y: usize, c: char) {
        if x < self.width && y < self.height {
            self.data[y * self.width + x].store(c as u8, Ordering::Relaxed);
        }
    }

    pub fn set_color(&self, color: Color, out: &mut impl std::io::Write) {
        queue!(out, SetForegroundColor(color)).unwrap();
    }

    pub fn move_to(&self, x: usize, y: usize, out: &mut impl std::io::Write) {
        queue!(out, MoveTo(x as u16, y as u16)).unwrap();
    }

    pub fn render(&self, out: &mut impl std::io::Write) {
        // let mut output = String::new();
        // for y in 0..self.height {
        //     let start = y * self.width;
        //     let end = start + self.width;

        //     for ch in &self.data[start..end] {
        //         queue!(out, Print(ch.load(Ordering::Relaxed) as char)).unwrap();
        //         // output.push(ch.load(Ordering::Relaxed) as char);
        //     }
        //     queue!(out, Print('\n')).unwrap();
        //     // output.push('\n');
        // }

        // for (y, row) in self.data.chunks(self.width).enumerate() {
        //     queue!(out, MoveTo(0, y as u16)).unwrap();
        //     let line: String = row
        //         .iter()
        //         .map(|ch| ch.load(Ordering::Relaxed) as char)
        //         .collect();
        //     queue!(out, Print(line)).unwrap();
        // }
        // out.flush().unwrap();
    }
}

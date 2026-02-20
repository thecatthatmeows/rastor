use crossterm::{cursor::MoveTo, queue, style::Print};

/// The `FrameBuffer` struct was made to render text-based graphics *faster* than just telling the
/// terminal to move the cursor and print each character individually.
pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub data: Vec<char>,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let data = vec![' '; width * height];
        FrameBuffer {
            width,
            height,
            data,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, c: char) {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = c;
        }
    }

    pub fn render(&self, out: &mut impl std::io::Write) {
        // let mut output = String::new();
        // for y in 0..self.height {
        //     let start = y * self.width;
        //     let end = start + self.width;

        //     for ch in &self.data[start..end] {
        //         output.push(*ch);
        //     }
        //     output.push('\n');
        // }

        for (_y, row) in self.data.chunks(self.width).enumerate() {
            queue!(out, MoveTo(0, 0)).unwrap();
            for ch in row {
                queue!(out, Print(ch)).unwrap();
            }
            queue!(out, Print('\n')).unwrap();
        }
    }
}

use std::io::Write;

use crossterm::{cursor::MoveTo, queue};

use crate::{shapes::Shape, types::{pos2::Pos2, vec2::Vec2}, ui::{ElementState, UIElement}};

#[derive(Debug, Clone)]
pub struct Text {
    pos: Pos2,
    size: Vec2<f32>,
    content: String,
}

impl Text {
    pub fn new(pos: Pos2, size: Vec2<f32>, content: String) -> Self {
        Self { pos, size, content }
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }
}

impl UIElement for Text {
    fn pos(&self) -> Pos2 { self.pos }
    fn size(&self) -> Vec2<f32> { self.size }

    fn draw(&self) {
        let mut stdout = std::io::stdout().lock();
        let pos: Vec2<f32> = self.pos.into();

        // +1 to account for border, so that the border doesnt cover up the contents-
        queue!(stdout, MoveTo(pos.x as u16+1, pos.y as u16)).unwrap();
        stdout.write_all(self.content.as_bytes()).unwrap();
        stdout.flush().unwrap();
    }

    fn update(&mut self) {
        // No dynamic behavior for now, but this could be used for blinking text, etc.
    }
}
pub mod style;

use std::io::Write;
use crossterm::{cursor::MoveTo, queue, style::{SetBackgroundColor, SetForegroundColor}};
use crate::{shapes::Shape, types::{color::Color, pos2::Pos2, vec2::Vec2}, ui::{ElementState, UIElement, text::style::TextStyle}};

#[derive(Debug, Clone)]
pub struct Text {
    pub pos: Pos2,
    pub size: Vec2<f32>,
    pub content: String,
    pub text_style: Option<TextStyle>,
}

impl Text {
    pub fn new(pos: Pos2, size: Vec2<f32>, content: String) -> Self {
        Self {
            pos,
            size,
            content,
            text_style: None
        }
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

        let text_color = self.text_style.as_ref().map_or(Color::White, |style| style.fg_color);
        let bg_color = self.text_style.as_ref().map_or(Color::Black, |style| style.bg_color);
        // +1 to account for border, so that the border doesnt cover up the contents-
        queue!(stdout, SetForegroundColor(text_color), MoveTo(pos.x as u16+1, pos.y as u16)).unwrap();
        stdout.write_all(self.content.as_bytes()).unwrap();
        stdout.flush().unwrap();
    }

    fn update(&mut self) {
        // No dynamic behavior for now, but this could be used for blinking text, etc.
    }
}

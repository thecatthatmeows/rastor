pub mod style;

use crate::{types::{pos2::Pos2, vec2::Vec2}, ui::{UIElement, container::style::ContainerStyle, style::border::Border}};
pub struct UIContainer {
    pub pos: Pos2,
    pub size: Vec2<f32>,
    pub children: Vec<Box<dyn UIElement>>,
    pub style: Option<ContainerStyle>,
}

impl UIContainer {
    pub fn new(pos: Pos2, size: Vec2<f32>) -> Self {
        Self {
            pos, size,
            children: Vec::new(),
            style: None
        }
    }

    pub fn add_child(&mut self, child: Box<dyn UIElement>) {
        self.children.push(child);
    }
}

impl UIElement for UIContainer {
    fn draw(&self) {
        for child in &self.children {
            child.draw();
        }

        if let Some(style) = &self.style {
            match style {
                ContainerStyle { border } => {
                    if let Some(border) = border {
                        border.draw();
                    }
                }
            }
        }
    }

    fn update(&mut self) {
        for child in &mut self.children {
            child.update();
        }
    }

    fn pos(&self) -> Pos2 {
        self.pos
    }

    fn size(&self) -> Vec2<f32> {
        self.size
    }
}

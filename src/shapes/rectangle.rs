use std::f32::consts::PI;

use crossterm::style::Color;

use crate::{shapes::{Orientation, line::Line, triangle::Triangle}, types::vec2::Vec2};

pub struct Rectangle {
    pub pos: Vec2<f32>,
    pub size: Vec2<f32>,
    triangles: [Triangle; 2],
    color: Color,
}

impl Rectangle {
    pub fn new(pos: Vec2<f32>, size: Vec2<f32>, color: Color) -> Self {
        let mut upper = Triangle::new(pos, Orientation::Left, size, color);
        let mut bottom = Triangle::new(pos, Orientation::Right, size, color);
        // upper.vertices.bottom_right += size;
        // bottom.vertices.bottom_right += size;

        Self {
            pos,
            size,
            color,
            triangles: [
                upper,
                bottom
            ]
        }
    }

    pub fn update(&mut self) {
        let upper = Triangle::new(self.pos, Orientation::Left, self.size, self.color);
        let bottom = Triangle::new(self.pos, Orientation::Right, self.size, self.color);

        self.triangles = [upper, bottom];

        for triangle in &mut self.triangles {
            triangle.update();
        }
    }

    pub fn draw(&self) {
        for triangle in &self.triangles {
            triangle.draw();
        }
    }
}

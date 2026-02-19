use std::f32::consts::{FRAC_PI_4, PI};

use crossterm::style::Color;

use crate::{shapes::{Orientation, line::Line, triangle::Triangle}, types::vec2::Vec2};

pub struct Rectangle {
    pub pos: Vec2<f32>,
    pub size: Vec2<f32>,
    pub orientation: Orientation,
    triangles: [Triangle; 2],
    color: Color,
}

impl Rectangle {
    pub fn new(pos: Vec2<f32>, size: Vec2<f32>, color: Color) -> Self {
        let orientation = Orientation::Left;
        let upper = Triangle::new(pos, orientation, size, color);
        let bottom = Triangle::new(pos, orientation.opposite(), size, color);
        // upper.vertices.bottom_right += size;
        // bottom.vertices.bottom_right += size;

        Self {
            pos,
            size,
            color,
            orientation,
            triangles: [
                upper,
                bottom
            ]
        }
    }

    pub fn update(&mut self) {
        let upper = Triangle::new(self.pos, self.orientation, self.size, self.color);
        let bottom = Triangle::new(self.pos, self.orientation.opposite(), self.size, self.color);

        self.triangles = [upper, bottom];

        for triangle in &mut self.triangles {
            triangle.update();
        }
    }

    pub fn rotate(&mut self, rad: f32) {
        self.triangles[0].orientation = Orientation::Custom(rad);
        self.triangles[1].orientation = Orientation::Custom(rad).opposite();
    }

    pub fn draw(&self) {
        for triangle in &self.triangles {
            triangle.draw();
        }
    }
}

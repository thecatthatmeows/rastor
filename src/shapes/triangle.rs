use std::{f32::consts::{FRAC_PI_4, PI}, fmt::format, io::stdout};

use crossterm::{queue, style::{Color, Print}};

use crate::{shapes::{Orientation, line::Line}, types::vec2::Vec2};

pub struct Triangle {
    pub vertices: TriangleVertices,
    pub orientation: Orientation,
    pub center: Vec2<f32>,
    lines: [Line; 3],
    color: Color,
}

#[derive(Clone, Copy)]
pub struct TriangleVertices {
    pub top_left: Vec2<f32>,
    pub bottom_left: Vec2<f32>,
    pub bottom_right: Vec2<f32>,
}

impl From<&[Vec2<f32>; 3]> for TriangleVertices {
    /// Args in order: top_left, bottom_left, bottom_right
    fn from(value: &[Vec2<f32>; 3]) -> Self {
        Self {
            top_left: value[0],
            bottom_left: value[1],
            bottom_right: value[2]
        }
    }
}

impl Triangle {
    pub fn new(center: Vec2<f32>, orientation: Orientation, size: Vec2<f32>, color: Color) -> Self {
        let p1 = Vec2::new(-1.0,  1.0) * size; // top left
        let p2 = Vec2::new(-1.0, -1.0) * size; // bottom left
        let p3 = Vec2::new( 1.0, -1.0) * size; // bottom right
        let vertices = &TriangleVertices::from(&[p1, p2, p3]);

        Self {
            vertices: *vertices,
            orientation,
            center,
            lines: [
                Line::new(p1, p2, color),
                Line::new(p2, p3, color),
                Line::new(p3, p1, color),
            ],
            color
        }
    }

    pub fn update(&mut self) {
        self.update_geometry();
    }

    fn update_geometry(&mut self) {
        let rad = match self.orientation {
            Orientation::Up => 0.0,
            Orientation::Right => PI/2.0, // 90
            Orientation::Down => PI, // 180
            Orientation::Left => 3.0 * PI/2.0, // 270
            Orientation::Custom(v) => v
        };
        let x_scale = match self.orientation {
            Orientation::Right | Orientation::Left => 2.0,
            _ => 1.0
        };

        let top_left = self.vertices.top_left; // top left
        let bottom_left = self.vertices.bottom_left; // bottom left
        let bottom_right = self.vertices.bottom_right; // bottom right

        let mut rp1 = top_left.rotate(rad);
        let mut rp2 = bottom_left.rotate(rad);
        let mut rp3 = bottom_right.rotate(rad);
        rp1.x *= x_scale;
        rp2.x *= x_scale;
        rp3.x *= x_scale;

        let sp1 = Self::to_screen_coords(rp1, self.center);
        let sp2 = Self::to_screen_coords(rp2, self.center);
        let sp3 = Self::to_screen_coords(rp3, self.center);

        self.lines = [
            Line::new(sp1, sp2, self.color),
            Line::new(sp2, sp3, self.color),
            Line::new(sp3, sp1, self.color),
        ];
    }

    fn to_screen_coords(v: Vec2<f32>, center: Vec2<f32>) -> Vec2<f32> {
        Vec2::new(v.x + center.x, -v.y + center.y)
    }

    pub fn draw(&self) {
        for line in &self.lines {
            line.draw();
        }
    }
}

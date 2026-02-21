use std::f32::consts::PI;

use crate::types::vec2::Vec2;

pub mod circle;
pub mod line;
pub mod rectangle;
pub mod triangle;

#[derive(Clone, Copy)]
pub enum Orientation {
    Up,
    Down,
    Left,
    Right,
    Custom(f32),
}

impl Orientation {
    pub fn opposite(&self) -> Self {
        match self {
            Orientation::Up => Orientation::Down,
            Orientation::Down => Orientation::Up,
            Orientation::Left => Orientation::Right,
            Orientation::Right => Orientation::Left,
            Orientation::Custom(v) => Orientation::Custom((v + PI) % (2.0 * PI)),
        }
    }

    pub fn to_f32(&self) -> f32 {
        match self {
            Orientation::Up => 0.0,
            Orientation::Down => PI,
            Orientation::Left => PI / 2.0,
            Orientation::Right => 3.0 * PI / 2.0,
            Orientation::Custom(v) => *v,
        }
    }
}

pub trait Shape {
    fn draw(&mut self);
    fn update(&mut self);
    fn set_orientation(&mut self, orientation: Orientation);
    fn orientation(&self) -> Orientation;
    fn rotate_to(&mut self, rad: f32) {
        self.set_orientation(Orientation::Custom(rad));
    }
    fn rotate(&mut self, rad: f32) {
        let last_rad = self.orientation().to_f32();
        let new_rad = last_rad + rad;
        self.set_orientation(Orientation::Custom(new_rad));
    }
}

/// if it outputs 0.0, its inside the triangle
pub fn edge(a: Vec2<f32>, b: Vec2<f32>, p: Vec2<f32>) -> f32 {
    (p.x - a.x) * (b.y - a.y) - (p.y - a.y) * (b.x - a.x)
}

pub fn inside_triangle(a: Vec2<f32>, b: Vec2<f32>, c: Vec2<f32>, p: Vec2<f32>) -> bool {
    let e1 = edge(a, b, p);
    let e2 = edge(b, c, p);
    let e3 = edge(c, a, p);

    (e1 >= 0.0 && e2 >= 0.0 && e3 >= 0.0) || (e1 <= 0.0 && e2 <= 0.0 && e3 <= 0.0)
}

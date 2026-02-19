use std::f32::consts::PI;

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
}

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

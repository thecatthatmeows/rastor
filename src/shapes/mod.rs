use std::f32::consts::PI;

use crate::{shapes::pixel::Pixel, types::{pos2::Pos2, vec2::Vec2}};

pub mod circle;
pub mod line;
pub mod rectangle;
pub mod triangle;
pub mod pixel;

use std::io::{StdoutLock, Write};
use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, SetForegroundColor},
};

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

/// Trait representing drawable/updatable shapes.
///
/// New method: `rasterize` allows shapes to emit `Pixel`s into a shared buffer
/// instead of performing terminal IO themselves. A default no-op implementation
/// is provided so existing shapes compile until they are updated to emit
/// pixels. Implementations should push pixels corresponding to their coverage
/// into `out`.
pub trait Shape {
    fn draw(&mut self);
    fn update(&mut self);
    fn set_orientation(&mut self, orientation: Orientation);
    fn orientation(&self) -> Orientation;

    /// Primary position of the shape. Implementations should return a Pos2
    /// representing the logical position of the shape (e.g. center for a
    /// `Circle`/`Triangle`, the `pos` field for a `Rectangle`, or a midpoint for
    /// a `Line`). and also accounting for absolute and relative position.
    fn pos(&self) -> Pos2;

    /// z-index used when ordering shapes for rendering. Lower values are drawn first.
    fn z_index(&self) -> i32;

    /// Return a boxed clone of this shape. This is required for cloning
    /// `Box<dyn Shape>` trait objects.
    fn box_clone(&self) -> Box<dyn Shape>;

    /// Rasterize this shape into the provided pixel buffer. `term_size` is the
    /// current terminal dimensions as `(width, height)` in u16. The default
    /// implementation is a no-op to preserve backward compatibility; update
    /// concrete shapes to push `Pixel`s here for batched rendering.
    fn rasterize(&self, _out: &mut Vec<Pixel>, _term_size: (u16, u16)) {
        // default no-op
    }

    fn rotate_to(&mut self, rad: f32) {
        self.set_orientation(Orientation::Custom(rad));
    }
    fn rotate(&mut self, rad: f32) {
        let last_rad = self.orientation().to_f32();
        let new_rad = last_rad + rad;
        self.set_orientation(Orientation::Custom(new_rad));
    }

    /// Convert a child's local (relative) position into the parent's world (absolute)
    /// coordinate space. Default implementation applies the parent's translation
    /// only (no rotation): the local coordinate is treated as an offset from the
    /// parent's `pos()`.
    ///
    /// Use this when you store a child's position relative to its parent and need
    /// the absolute position for rendering, collision checks, etc.
    fn local_to_parent(&self, local: Pos2) -> Pos2 {
        local + self.pos()
    }

    /// Convert a world (absolute) position into this shape's local (relative)
    /// coordinate space. This is the inverse of `local_to_parent` for the
    /// translation-only transform.
    fn parent_to_local(&self, world: Pos2) -> Pos2 {
        world - self.pos()
    }

    fn collides_with(&self, other: &dyn Shape) -> bool;

    /// Set the parent's absolute position for this shape. Default is a no-op.
    /// Parent shapes should call this on their children during their own
    /// `update`/`draw` so child shapes can compute absolute positions from
    /// their local coordinates.
    fn set_parent_pos(&mut self, pos: Pos2) {}
}

/// Allow cloning boxed trait objects: `Box<dyn Shape>`.
impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Box<dyn Shape> {
        self.box_clone()
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

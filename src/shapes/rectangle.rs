use crate::{
    shapes::{Orientation, Shape, inside_triangle, triangle::Triangle},
    types::{pos2::Pos2, vec2::Vec2},
};
use crossterm::style::Color;

pub struct Rectangle {
    pub pos: Pos2,
    pub size: Vec2<f32>,
    pub orientation: Orientation,
    pub color: Color,
    pub z_index: i32,
    triangles: [Triangle; 2],
    pub children: Vec<Box<dyn Shape>>,
}

impl Rectangle {
    pub fn new(pos: Pos2, mut size: Vec2<f32>, color: Color) -> Self {
        let orientation = Orientation::Left;
        size.swap();
        let upper = Triangle::new(pos, orientation, size, color);
        let bottom = Triangle::new(pos, orientation.opposite(), size, color);
        // upper.vertices.bottom_right += size;
        // bottom.vertices.bottom_right += size;

        Self {
            pos,
            size,
            orientation,
            color,
            z_index: 0,
            triangles: [upper, bottom],
            children: vec![],
        }
    }

    pub fn push(&mut self, child: Box<dyn Shape>) {
        self.children.push(child);
    }

    /// Inherent getter for this rectangle's z-index. This is useful when you
    /// have a concrete `Rectangle` (e.g. `Vec<Rectangle>`) and want to sort it.
    pub fn z_index(&self) -> i32 {
        self.z_index
    }
}

impl Clone for Rectangle {
    fn clone(&self) -> Self {
        // Recreate the triangles rather than attempting to clone Triangle directly
        // (Triangle contains a StdoutLock which is not Clone). Children are cloned
        // via the Box<dyn Shape> Clone implementation (requires each concrete
        // shape to implement `box_clone`).
        let upper = Triangle::new(self.pos, self.orientation, self.size, self.color);
        let bottom = Triangle::new(self.pos, self.orientation.opposite(), self.size, self.color);

        Self {
            pos: self.pos,
            size: self.size,
            orientation: self.orientation,
            color: self.color,
            z_index: self.z_index,
            triangles: [upper, bottom],
            children: self.children.iter().map(|c| c.clone()).collect(),
        }
    }
}

impl Shape for Rectangle {
    fn update(&mut self) {
        let upper = Triangle::new(self.pos, self.orientation, self.size, self.color);
        let bottom = Triangle::new(self.pos, self.orientation.opposite(), self.size, self.color);

        self.triangles = [upper, bottom];
        self.triangles.sort_by_key(|triangle| triangle.z_index);

        for triangle in &mut self.triangles {
            triangle.update();
        }

        self.children.sort_by_key(|child| child.z_index());

        let parent_pos: Vec2<f32> = self.pos().into();
        for child in &mut self.children {
            let relative_pos = child.pos().to_relative(parent_pos);

            if let Pos2::Relative(_) = relative_pos {
                child.set_pos(relative_pos);
            }

            child.update();
        }
    }

    fn draw(&mut self) {
        for triangle in &mut self.triangles {
            triangle.draw();
        }

        self.children.sort_by_key(|child| child.z_index());

        // is this considered recursive or..??
        for child in &mut self.children {
            child.draw();
        }
    }

    fn set_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }

    fn orientation(&self) -> Orientation {
        self.orientation
    }

    fn box_clone(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }

    fn z_index(&self) -> i32 {
        self.z_index
    }

    fn collides_with(&self, other: &dyn Shape) -> bool {
        // Build temporary triangles representing this rectangle (upper and bottom),
        // update their geometry (which applies orientation), and then test whether
        // the other's position lies inside either triangle.
        //
        // This approach correctly handles rotated rectangles because the
        // Triangle::update_geometry applies rotation and converts to screen coords.
        let mut upper = Triangle::new(self.pos, self.orientation, self.size, self.color);
        let mut bottom = Triangle::new(self.pos, self.orientation.opposite(), self.size, self.color);
        upper.update();
        bottom.update();

        let p = other.pos();
        let up_v = upper.vertices.to_arr();
        let bot_v = bottom.vertices.to_arr();

        inside_triangle(up_v[0], up_v[1], up_v[2], p.into()) ||
        inside_triangle(bot_v[0], bot_v[1], bot_v[2], p.into())
    }

    fn pos(&self) -> Pos2 {
        self.pos
    }

    fn set_pos(&mut self, pos: Pos2) {
        // Set this rectangle's primary position.
        // This updates the rectangle's stored `pos` (its logical/world position).
        self.pos = pos;
    }
}

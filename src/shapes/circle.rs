use std::f32::consts::PI;

use crossterm::style::Color;

use crate::{
    shapes::{Orientation, Shape, triangle::Triangle},
    types::vec2::Vec2,
};

pub struct Circle {
    pub center: Vec2<f32>,
    pub radius: f32,
    pub orientation: Orientation,
    pub color: Color,
    pub z_index: i32,
    triangles: Vec<Triangle>,
}

impl Circle {
    pub fn new(center: Vec2<f32>, radius: f32, n_sectors: usize, color: Color) -> Self {
        let mut triangles = Vec::new();

        let angle_per_triangle = (PI * 2.0) / n_sectors as f32;
        let base_length = 2.0 * radius * (angle_per_triangle / 2.0).sin();

        for i in 0..n_sectors {
            let theta = i as f32 * (2.0 * std::f32::consts::PI) / n_sectors as f32;

            let triangle = Triangle::new(
                center,
                Orientation::Custom(theta),
                Vec2::new(radius, base_length),
                color,
            );
            triangles.push(triangle);
        }

        // Ensure triangles are ordered by their z_index so rendering order is stable.
        triangles.sort_by_key(|t| t.z_index);

        Self {
            center,
            radius,
            orientation: Orientation::Custom(0.0),
            color,
            z_index: 0,
            triangles,
        }
    }

    /// Getter for z_index as an inherent method.
    pub fn z_index(&self) -> i32 {
        self.z_index
    }
}

impl Clone for Circle {
    fn clone(&self) -> Self {
        // Triangles (and their stdout locks) are not directly cloneable.
        // Recreate the triangles array using the same radius and sector count.
        let n_sectors = self.triangles.len();
        let mut c = Circle::new(self.center, self.radius, n_sectors, self.color);
        c.orientation = self.orientation;
        c.z_index = self.z_index;
        c
    }
}

impl Shape for Circle {
    fn draw(&mut self) {
        for triangle in &mut self.triangles {
            triangle.draw();
        }
    }

    fn update(&mut self) {
        // Update geometry/state of each triangle first.
        for triangle in &mut self.triangles {
            triangle.update();
        }

        // After updates, re-sort by z_index so rendering order respects z values.
        self.triangles.sort_by_key(|t| t.z_index);
    }

    fn set_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }

    fn orientation(&self) -> Orientation {
        self.orientation
    }

    fn z_index(&self) -> i32 {
        self.z_index
    }

    fn pos(&self) -> Vec2<f32> {
        // The logical position for a circle is its center.
        self.center
    }

    fn box_clone(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }

    fn collides_with(&self, other: &dyn Shape) -> bool {
        // Point-to-circle collision: check whether the other's position lies within
        // this circle's radius. We use squared distance to avoid an expensive sqrt.
        let other_pos = other.pos();
        let dx = other_pos.x - self.center.x;
        let dy = other_pos.y - self.center.y;
        (dx * dx + dy * dy) <= (self.radius * self.radius)
    }
}

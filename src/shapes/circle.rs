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

        Self {
            center,
            radius,
            orientation: Orientation::Custom(0.0),
            color,
            z_index: 0,
            triangles,
        }
    }
}

impl Shape for Circle {
    fn draw(&mut self) {
        // let mut triangles = Vec::new();
        // for i in 0..self.triangles.len() {
        //     let theta = i as f32 * (2.0 * std::f32::consts::PI) / self.triangles.len() as f32;
        //     let next_theta =
        //         (i + 1) as f32 * (2.0 * std::f32::consts::PI) / self.triangles.len() as f32;

        //     let p1 = self.center;
        //     let p2 = self.center
        //         + Vec2 {
        //             x: self.radius * theta.cos(),
        //             y: self.radius * theta.sin(),
        //         };
        //     let p3 = self.center
        //         + Vec2 {
        //             x: self.radius * next_theta.cos(),
        //             y: self.radius * next_theta.sin(),
        //         };

        //     let mut triangle =
        //         Triangle::new(p1, Orientation::Right, Vec2::splat(self.radius), self.color);
        //     // triangle.base_vertices.bottom_left = p1;
        //     // triangle.base_vertices.bottom_right = p2;
        //     // triangle.base_vertices.top_left = p3;
        //     triangles.push(triangle);
        // }
        // self.triangles = triangles;

        for triangle in &mut self.triangles {
            triangle.draw();
        }
    }

    fn update(&mut self) {
        for triangle in &mut self.triangles {
            triangle.update();
        }
    }

    fn set_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }

    fn orientation(&self) -> Orientation {
        self.orientation
    }
}

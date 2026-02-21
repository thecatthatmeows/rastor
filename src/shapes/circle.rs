use crossterm::style::Color;

use crate::{
    shapes::{Orientation, Shape, triangle::Triangle},
    types::vec2::Vec2,
};

pub struct Circle {
    pub pos: Vec2<f32>,
    pub size: Vec2<f32>,
    pub orientation: Orientation,
    pub color: Color,
    pub z_index: i32,
    triangles: [Triangle; 2],
}

impl Shape for Circle {
    fn draw(&mut self) {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    fn rotate_to(&mut self, rad: f32) {
        todo!()
    }

    fn rotate(&mut self, rad: f32) {
        todo!()
    }

    fn set_orientation(&mut self, orientation: Orientation) {
        todo!()
    }

    fn orientation(&self) -> Orientation {
        todo!()
    }
}

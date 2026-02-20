use std::{f32::consts::{FRAC_PI_2, FRAC_PI_4, PI}, fmt::format, io::stdout};

use crossterm::{cursor::MoveTo, queue, style::{Color, Print, SetForegroundColor}, terminal};

use crate::{shapes::{Orientation, inside_triangle, line::Line}, types::vec2::Vec2};

pub struct Triangle {
    pub base_vertices: TriangleVertices,
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

impl TriangleVertices {
    pub fn to_arr(&self) -> [Vec2<f32>; 3] {
        [self.top_left, self.bottom_left, self.bottom_right]
    }
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
        let base_vertices = TriangleVertices::from(&[p1, p2, p3]);

        Self {
            base_vertices,
            vertices: base_vertices,
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

    fn fill_color(&self) {
        let mut stdout = stdout().lock();
        queue!(stdout, SetForegroundColor(self.color)).unwrap();

        let (term_width, term_height) = terminal::size().unwrap();

        let vertices = self.vertices.to_arr();
        // let screen_vertices = [
        //     vertices[0] + center,
        //     vertices[1] + center,
        //     vertices[2] + center,
        // ];
        let mut x = vec![];
        let mut y = vec![];

        for vertex in vertices {
            x.push(vertex.x as i32);
            y.push(vertex.y as i32);
        }

        let min_x = vertices.iter().map(|v| v.x as i32).min().unwrap();
        let max_x = vertices.iter().map(|v| v.x as i32).max().unwrap();
        let min_y = vertices.iter().map(|v| v.y as i32).min().unwrap();
        let max_y = vertices.iter().map(|v| v.y as i32).max().unwrap();

        let mut buf = Vec::with_capacity(1024);
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Vec2::new(x as f32, y as f32);

                if inside_triangle(vertices[0], vertices[1], vertices[2], p) {
                    let screen_x = (x) as u16;
                    let screen_y = (y) as u16;

                    if screen_x < term_width && screen_y < term_height {
                        buf.push((screen_x, screen_y));
                    }
                }
            }
        }

        for (x, y) in buf {
            queue!(
                stdout,
                MoveTo(x, y),
                Print("█")
            ).unwrap();
        }
    }

    fn update_geometry(&mut self) {
        let rad = self.rad();
        let x_scale = 2.0;

        let top_left = self.base_vertices.top_left; // top left
        let bottom_left = self.base_vertices.bottom_left; // bottom left
        let bottom_right = self.base_vertices.bottom_right; // bottom right

        let mut rp1 = top_left.rotate(rad);
        let mut rp2 = bottom_left.rotate(rad);
        let mut rp3 = bottom_right.rotate(rad);
        rp1.x *= x_scale;
        rp2.x *= x_scale;
        rp3.x *= x_scale;

        // self.vertices.top_left = rp1;
        // self.vertices.bottom_left = rp2;
        // self.vertices.bottom_right = rp3;

        let sp1 = Self::to_screen_coords(rp1, self.center);
        let sp2 = Self::to_screen_coords(rp2, self.center);
        let sp3 = Self::to_screen_coords(rp3, self.center);

        self.vertices.top_left = sp1;
        self.vertices.bottom_left = sp2;
        self.vertices.bottom_right = sp3;

        self.lines = [
            Line::new(sp1, sp2, self.color),
            Line::new(sp2, sp3, self.color),
            Line::new(sp3, sp1, self.color),
        ];
    }

    pub fn rad(&self) -> f32 {
        match self.orientation {
            Orientation::Up => 0.0,
            Orientation::Right => PI/2.0, // 90
            Orientation::Down => PI, // 180
            Orientation::Left => 3.0 * FRAC_PI_2, // 270
            Orientation::Custom(v) => v
        }
    }

    fn to_screen_coords(v: Vec2<f32>, center: Vec2<f32>) -> Vec2<f32> {
        Vec2::new(v.x + center.x, -v.y + center.y)
    }

    pub fn draw(&self) {
        self.fill_color();
        for line in &self.lines {
            line.draw();
        }
    }
}

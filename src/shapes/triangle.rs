use crate::{
    shapes::{Orientation, Shape, inside_triangle, line::Line, pixel::{Pixel, flush_pixels}},
    types::{pos2::Pos2, vec2::Vec2},
};
use crossterm::style::Color;
use crossterm::terminal;
use std::f32::consts::{FRAC_PI_2, PI};

pub struct Triangle {
    pub base_vertices: TriangleVertices,
    pub vertices: TriangleVertices,
    pub orientation: Orientation,
    /// Local center is the logical position relative to a parent. When the
    /// triangle is top-level, `local_center == center`.
    pub local_center: Pos2,
    /// `center` is the absolute/world center used for geometry updates and rendering.
    pub center: Pos2,
    pub z_index: i32,
    pub color: Color,
    lines: Vec<Line>,
}

#[derive(Clone, Copy, Debug)]
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
            bottom_right: value[2],
        }
    }
}

impl Triangle {
    pub fn new(center: Pos2, orientation: Orientation, size: Vec2<f32>, color: Color) -> Self {
        let p1 = Vec2::new(-1.0, 1.0) * size; // top left
        let p2 = Vec2::new(-1.0, -1.0) * size; // bottom left
        let p3 = Vec2::new(1.0, -1.0) * size; // bottom right
        let base_vertices = TriangleVertices::from(&[p1, p2, p3]);

        // By default, treat the provided `center` as the local center. For a
        // top-level shape this is also its world `center`. When parented, the
        // parent will call `set_parent_pos` to update the absolute `center`.
        Self {
            base_vertices,
            vertices: base_vertices,
            orientation,
            local_center: center,
            center,
            lines: vec![
                Line::new(p1, p2, color),
                Line::new(p2, p3, color),
                Line::new(p3, p1, color),
            ],
            color,
            z_index: 0,
        }
    }

    /// Rasterize triangle interior into the output pixel buffer.
    /// `term_size` is (width, height) in terminal cells.
    fn rasterize(&self, out: &mut Vec<Pixel>, term_size: (u16, u16)) {
        let (term_width, term_height) = (term_size.0 as i32, term_size.1 as i32);

        let vertices = self.vertices.to_arr();

        let min_x = vertices.iter().map(|v| v.x as i32).min().unwrap();
        let max_x = vertices.iter().map(|v| v.x as i32).max().unwrap();
        let min_y = vertices.iter().map(|v| v.y as i32).min().unwrap();
        let max_y = vertices.iter().map(|v| v.y as i32).max().unwrap();

        // Pre-size a bit to avoid repeated reallocations for larger triangles.
        out.reserve(((max_x - min_x + 1) * (max_y - min_y + 1)) as usize);

        for py in min_y..=max_y {
            for px in min_x..=max_x {
                let p = Vec2::new(px as f32, py as f32);
                if inside_triangle(vertices[0], vertices[1], vertices[2], p) {
                    if px >= 0 && py >= 0 && px < term_width && py < term_height {
                        out.push(Pixel::new(
                            px as u16,
                            py as u16,
                            '█',
                            self.color,
                            self.z_index,
                        ));
                    }
                }
            }
        }
    }

    fn update_geometry(&mut self) {
        let rad = self.rad();
        let x_scale = 2.2;

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

        let center_vec: Vec2<f32> = self.center.into();
        let sp1 = Self::to_screen_coords(rp1, center_vec);
        let sp2 = Self::to_screen_coords(rp2, center_vec);
        let sp3 = Self::to_screen_coords(rp3, center_vec);

        self.vertices.top_left = sp1;
        self.vertices.bottom_left = sp2;
        self.vertices.bottom_right = sp3;

        self.lines = vec![
            Line::new(sp1, sp2, self.color),
            Line::new(sp2, sp3, self.color),
            Line::new(sp3, sp1, self.color),
        ];
        // Sort line draw order deterministically by their vertical midpoint so
        // overlapping/overdraw can be consistent. Lower midpoint (smaller y)
        // will be drawn first.
        self.lines.sort_by_key(|line| {
            let line_pos1: Vec2<f32> = line.pos1.into();
            let line_pos2: Vec2<f32> = line.pos2.into();

            (line_pos1.y + line_pos2.y) as i32
        });
    }

    pub fn rad(&self) -> f32 {
        match self.orientation {
            Orientation::Up => 0.0,
            Orientation::Right => PI / 2.0,       // 90
            Orientation::Down => PI,              // 180
            Orientation::Left => 3.0 * FRAC_PI_2, // 270
            Orientation::Custom(v) => v,
        }
    }

    fn to_screen_coords(v: Vec2<f32>, center: Vec2<f32>) -> Vec2<f32> {
        Vec2::new(v.x + center.x, -v.y + center.y)
    }
}

impl Shape for Triangle {
    fn draw(&mut self) {
        // Sort lines deterministically (same as before) so line order is consistent.
        self.lines.sort_by_key(|line| {
            let line_pos1: Vec2<f32> = line.pos1.into();
            let line_pos2: Vec2<f32> = line.pos2.into();

            (line_pos1.y + line_pos2.y) as i32
        });

        // Rasterize the filled interior into a temporary pixel buffer and flush
        // it in a single batched write.
        let (w, h) = terminal::size().unwrap();
        let mut pixels: Vec<Pixel> = Vec::with_capacity(1024);
        self.rasterize(&mut pixels, (w, h));
        let mut out = std::io::stdout().lock();
        flush_pixels(&mut out, &mut pixels);

        // Draw border lines on top (each `Line::draw` will rasterize and flush).
        for line in &mut self.lines {
            line.draw();
        }
    }

    fn update(&mut self) {
        self.update_geometry();
    }

    fn set_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }

    fn orientation(&self) -> Orientation {
        self.orientation
    }

    fn pos(&self) -> Pos2 {
        // The logical position for a triangle (for parenting) is its local center.
        // Parents will compute the child's world position via `local_to_parent(child.pos())`
        // and then call `set_parent_pos` with that absolute position.
        self.local_center
    }

    fn set_parent_pos(&mut self, parent_pos: Pos2) {
        // The parent provides the *absolute* world position for this shape.
        // Store it in `center`, which is used for geometry updates and rendering.
        self.center = parent_pos;
    }

    fn z_index(&self) -> i32 {
        self.z_index
    }

    fn box_clone(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }

    fn collides_with(&self, other: &dyn Shape) -> bool {
        let p = other.pos();
        let verts = self.vertices.to_arr();
        inside_triangle(verts[0], verts[1], verts[2], p.into())
    }
}

impl Clone for Triangle {
    fn clone(&self) -> Self {
        let verts = self.vertices.to_arr();
        Self {
            base_vertices: self.base_vertices,
            vertices: self.vertices,
            orientation: self.orientation,
            local_center: self.local_center,
            center: self.center,
            z_index: self.z_index,
            color: self.color,
            lines: vec![
                Line::new(verts[0], verts[1], self.color),
                Line::new(verts[1], verts[2], self.color),
                Line::new(verts[2], verts[0], self.color),
            ],
        }
    }
}

use std::ops::{Add, AddAssign, Div, Mul, Sub};
use num_traits::{Float, ToPrimitive};

#[derive(Debug, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: AddAssign> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Add for Vec2<T>
where
    T: Add<Output = T>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Vec2<T>
where
    T: Sub<Output = T>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul for Vec2<T>
where
    T: Mul<Output = T>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T> Div for Vec2<T>
where
    T: Div<Output = T>
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}


impl<T: Float> Vec2<T> {
    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn cross(self, rhs: Self) -> T {
        self.x * rhs.y - self.y * rhs.x
    }

    pub fn rotate(self, rad: T) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();

        Self {
            x: self.x * cos + self.y * sin,
            y: -self.x * sin + self.y * cos,
        }
    }
}

impl<T> Vec2<T> 
where
    T: Copy + Add<Output = T> + ToPrimitive
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn splat(num: T) -> Self {
        Self {
            x: num,
            y: num,
        }
    }

    pub fn swap(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
    }

    pub fn to_f32(&self) -> Vec2<f32> {
        Vec2::<f32> {
            x: self.x.to_f32().unwrap(),
            y: self.y.to_f32().unwrap()
        }
    }
}

impl<T> From<&Vec2<T>> for Vec2<T>
where
    T: Copy
{
    fn from(value: &Vec2<T>) -> Self {
        *value
    }
}

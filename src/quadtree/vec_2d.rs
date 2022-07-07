use std::ops::Sub;

// A 2D Point
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64,
}

// Implement Subtraction between vectors
impl Sub for Vec2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Vec2D {

    pub fn norm(&self) -> f64 {
        (self.x + self.y).powi(2).sqrt()
    }

}

// See https://doc.rust-lang.org/std/cmp/trait.Eq.html
impl Eq for Vec2D { }

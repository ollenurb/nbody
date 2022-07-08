use std::default::Default;
use std::ops::{Add, Sub};

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

impl Add for Vec2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Vec2D {
    pub fn norm(&self) -> f64 {
        (self.x + self.y).powi(2).sqrt()
    }
}

impl Default for Vec2D {
    fn default() -> Self {
        Vec2D { x: 0.0, y: 0.0 }
    }
}

// See https://doc.rust-lang.org/std/cmp/trait.Eq.html
impl Eq for Vec2D {}

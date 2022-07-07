// A 2D Point
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64,
}

// See https://doc.rust-lang.org/std/cmp/trait.Eq.html
impl Eq for Vec2D { }

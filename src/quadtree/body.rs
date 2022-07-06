use super::Point;

#[derive(Debug, Clone, Copy)]
pub struct Body {
    pub position: Point,
    pub total_mass: f32,
}


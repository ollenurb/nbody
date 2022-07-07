use crate::consts::G;

use super::Vec2D;

use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Body {
    pub position: Vec2D,
    pub velocity: Vec2D,
    pub mass: f64,
}

impl Body {

    pub fn from_random(w: f64, h: f64, mass: f64) -> Self {
        let mut rng = rand::thread_rng();
        Body {
            position: Vec2D { x: rng.gen_range(0.0..w), y: rng.gen_range(0.0..h) },
            mass,
            velocity: Vec2D { x: 0.0, y: 0.0 },
        }
    }

    // Euclidean distance between two bodies
    // Defined as ||p - q||_2
    pub fn dist(&self, other: &Body) -> f64 {
        (self.position - other.position).norm()
    }

    // Compute the exerted force with respect to another Body
    pub fn force(&self, b: &Body) -> f64 {
        let a = self;
        (G * a.mass * b.mass) / a.dist(b).powi(2)
    }

}

impl Default for Body {
    fn default() -> Self {
        Body {
            position: Vec2D { x: 0.0, y: 0.0 },
            mass: 0.0,
            velocity: Vec2D { x: 0.0, y: 0.0 },
        }
    }
}



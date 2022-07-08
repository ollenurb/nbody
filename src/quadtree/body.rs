use crate::consts::G;

use super::Vec2D;

use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Body {
    pub position: Vec2D,
    pub velocity: Vec2D,
    pub mass: f64,
    pub force: Vec2D,
}

impl Body {
    pub fn from_random(w: f64, h: f64, mass: f64) -> Self {
        let mut rng = rand::thread_rng();
        Body {
            position: Vec2D {
                x: rng.gen_range(0.0..w),
                y: rng.gen_range(0.0..h),
            },
            mass,
            velocity: Default::default(),
            force: Default::default(),
        }
    }

    // Update position and velocity based on the current velocity and force exerted by other bodies
    pub fn update_position(&mut self) {
        self.velocity.x += self.force.x / self.mass;
        self.velocity.y += self.force.y / self.mass;

        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }

    pub fn reset_force(&mut self) {
        self.force.x = 0.0;
        self.force.y = 0.0;
    }

    // Euclidean distance between two bodies
    // Defined as ||p - q||_2
    pub fn dist(&self, other: &Body) -> f64 {
        (self.position - other.position).norm()
    }

    // Update the exerted force by another Body
    pub fn update_force(&mut self, b: &Body) {
        let dx = self.position.x - b.position.x;
        let dy = self.position.y - b.position.y;

        let r = (dx.powi(2) + dy.powi(2)).sqrt();
        let f = (G * self.mass * b.mass) / r.powi(2);

        self.force.x += (f * dx) / r;
        self.force.y += (f * dy) / r;
    }
}

impl Default for Body {
    fn default() -> Self {
        Body {
            position: Vec2D { x: 0.0, y: 0.0 },
            mass: 0.0,
            velocity: Vec2D { x: 0.0, y: 0.0 },
            force: Default::default(),
        }
    }
}

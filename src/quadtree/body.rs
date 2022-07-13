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

    pub fn new(position: Vec2D, velocity: Vec2D, mass: f64) -> Self {
        Body { position, velocity, mass, force: Default::default() }
    }

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
    pub fn update_position(&mut self, dt: f64) {
        self.velocity.x += dt * self.force.x / self.mass;
        self.velocity.y += dt * self.force.y / self.mass;

        self.position.x += dt * self.velocity.x;
        self.position.y += dt * self.velocity.y;
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
        let dx = b.position.x - self.position.x;
        let dy = b.position.y - self.position.y;
        let eps: f64 = 3e4;

        let r = (dx.powi(2) + dy.powi(2)).sqrt();
        let f = (G * self.mass * b.mass) / (r.powi(2) + eps.powi(2));

        self.force.x += f * dx / r;
        self.force.y += f * dy / r;
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

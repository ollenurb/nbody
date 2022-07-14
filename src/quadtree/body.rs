use crate::consts::{G, EPS};
use ultraviolet::DVec2;

#[derive(Debug, Clone, Copy)]
pub struct Body {
    pub position: DVec2,
    pub velocity: DVec2,
    pub mass: f64,
    pub force: DVec2,
}

impl Body {

    /// Creates a new Astronomical Body with the given initial position (rx, ry), velocity (vx,
    /// vy)) and mass
    pub fn new(rx: f64, ry: f64, vx: f64, vy: f64, mass: f64) -> Self {
        Body {
           position: DVec2 { x: rx, y: ry },
           velocity: DVec2 { x: vx, y: vy },
           mass,
           force: DVec2::default()
        }
    }

    // Update position and velocity based on the current velocity and force exerted by other bodies
    // To obtain velocity we just integrate the acceleration.
    // By Newton's second law of motion, (F = m * a), hence (a = F/m)
    pub fn update_position(&mut self, dt: f64) {
        // Integrate acceleration to obtain velocity
        self.velocity += dt * self.force / self.mass;
        // Integrate velocity to obtain position
        self.position += dt * self.velocity;
    }

    pub fn reset_force(&mut self) {
        self.force = DVec2::zero();
    }

    // Euclidean distance between two bodies
    // Defined as ||p - q||_2
    pub fn dist(&self, other: &Body) -> f64 {
        (self.position - other.position).mag()
    }

    // Update the exerted force by another Body
    pub fn update_force(&mut self, b: &Body) {
        let d = b.position - self.position;
        let r = d.mag();
        let f = (G * self.mass * b.mass) / (r.powi(2) + EPS.powi(2));

        self.force += f * d / r;
    }
}

impl Default for Body {
    fn default() -> Self {
        Body {
            position: DVec2::zero(),
            mass: 0.0,
            velocity: DVec2::zero(),
            force: DVec2::zero(),
        }
    }
}

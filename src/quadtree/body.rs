use super::Vec2D;

#[derive(Debug, Clone, Copy)]
pub struct Body {
    pub position: Vec2D,
    pub mass: f64,
}

impl Body {
    // Compute the exerted force with respect to another Body
    pub fn update_foces(&mut self, b: &Body) {
        let new_mass = self.mass + b.mass;
        self.position.x = ((self.position.x * self.mass) + (b.position.x * b.mass)) / new_mass;
        self.position.y = ((self.position.y * self.mass) + (b.position.y * b.mass)) / new_mass;
        self.mass = new_mass;
    }
}

impl Default for Body {
    fn default() -> Self {
        Body {
            position: Vec2D { x: 0.0, y: 0.0 },
            mass: 0.0,
        }
    }
}



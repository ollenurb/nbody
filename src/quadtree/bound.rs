use super::Body;

// A rectangle is represented with the top left corner point and its dimensions
#[derive(Debug, Clone)]
pub struct Bound {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Bound {
    // True if body is contained inside the Rectangle (self), False otherwise
    pub fn contains(&self, body: &Body) -> bool {
        (body.position.x < self.x + self.w)
            && (body.position.x > self.x)
            && (body.position.y < self.y + self.h)
            && (body.position.y > self.y)
    }

    pub fn subdivide(&self) -> (Bound, Bound, Bound, Bound) {
        let new_w = self.w / 2.0;
        let new_h = self.h / 2.0;

        let nw = Bound { x: self.x, y: self.y, w: new_w, h: new_h };
        let ne = Bound { x: self.x + new_w, y: self.y, w: new_w, h: new_h, };
        let sw = Bound { x: self.x + new_w, y: self.y + new_h, w: new_w, h: new_h };
        let se = Bound { x: self.x, y: self.y + new_h, w: new_w, h: new_h };

        (nw, ne, sw, se)
    }

}

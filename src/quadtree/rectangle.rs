use super::Point;

// A rectangle is represented with the top left corner point and its dimensions
#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub corner: Point,
    pub w: f64,
    pub h: f64,
}

impl Rectangle {
    // True if point is contained inside the Rectangle (self), False otherwise
    pub fn contains(&self, point: &Point) -> bool {
        (point.x < self.corner.x + self.w)
            && (point.x > self.corner.x)
            && (point.y < self.corner.y + self.h)
            && (point.y > self.corner.y)
    }

    pub fn subdivide(&self) -> (Rectangle, Rectangle, Rectangle, Rectangle) {
        let new_w = self.w / 2.0;
        let new_h = self.h / 2.0;
        let cur_x = self.corner.x;
        let cur_y = self.corner.y;

        (
            Rectangle {
                corner: Point { ..self.corner },
                w: new_w,
                h: new_h,
            },
            Rectangle {
                corner: Point {
                    x: cur_x + new_w,
                    y: cur_y,
                },
                w: new_w,
                h: new_h,
            },
            Rectangle {
                corner: Point {
                    x: cur_x + new_w,
                    y: cur_y + new_h,
                },
                w: new_w,
                h: new_h,
            },
            Rectangle {
                corner: Point {
                    x: cur_x,
                    y: cur_y + new_h,
                },
                w: new_w,
                h: new_h,
            },
        )
    }
}

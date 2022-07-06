#![allow(dead_code)]

use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub struct Body {
    pub position: Point,
    pub total_mass: f32,
}

// A 2D Point
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

// A rectangle is represented with the top left corner point and its dimensions
#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub corner: Point,
    pub w: f64,
    pub h: f64,
}

impl Rectangle {
    // True if point is contained inside the Rectangle (self), False otherwise
    fn contains(&self, point: &Point) -> bool {
        (point.x < self.corner.x + self.w)
            && (point.x > self.corner.x)
            && (point.y < self.corner.y + self.h)
            && (point.y > self.corner.y)
    }

    fn subdivide(&self) -> (Rectangle, Rectangle, Rectangle, Rectangle) {
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

#[derive(Debug, Clone)]
pub struct QuadTree {
    boundary: Rectangle,
    node: Node,
}

// A node can be either:
//  * Empty
//  * External
//  * Internal
#[derive(Debug, Clone)]
enum Node {
    Empty,

    External(Rc<Body>),

    Internal {
        nw: Box<QuadTree>,
        ne: Box<QuadTree>,
        sw: Box<QuadTree>,
        se: Box<QuadTree>,
    },
}

impl QuadTree {
    pub fn new(boundary: Rectangle) -> Self {
        QuadTree {
            boundary,
            node: Node::Empty,
        }
    }

    pub fn insert(&mut self, body: Rc<Body>) {
        match self.node {
            // If the node doesn't contain any node, then put the new body here
            Node::Empty => self.node = Node::External(body),

            // if the node is an internal node, insert it recursively into the appropriate quadrant
            Node::Internal {
                ref mut nw,
                ref mut ne,
                ref mut sw,
                ref mut se,
            } => {
                if nw.boundary.contains(&body.position) {
                    nw.insert(body)
                } else if ne.boundary.contains(&body.position) {
                    ne.insert(body)
                } else if sw.boundary.contains(&body.position) {
                    sw.insert(body)
                } else if se.boundary.contains(&body.position) {
                    se.insert(body)
                } else {
                    println!("Error: {:#?}", self.node);
                }
            }

            // If node x is an external node, say containing a body named c, then there are two
            // bodies b and c in the same region. Subdivide the region further by creating four
            // children. Then, recursively insert both b and c into the appropriate quadrant(s).
            // Since b and c may still end up in the same quadrant, there may be several
            // subdivisions during a single insertion. Finally, update the center-of-mass and total
            // mass of x.
            Node::External(ref c) => {
                let rc_clone = Rc::clone(c);
                let childs = self.boundary.subdivide();
                self.node = Node::Internal {
                    nw: Box::new(QuadTree {
                        boundary: childs.0,
                        node: Node::Empty,
                    }),
                    ne: Box::new(QuadTree {
                        boundary: childs.1,
                        node: Node::Empty,
                    }),
                    sw: Box::new(QuadTree {
                        boundary: childs.2,
                        node: Node::Empty,
                    }),
                    se: Box::new(QuadTree {
                        boundary: childs.3,
                        node: Node::Empty,
                    }),
                };
                self.insert(rc_clone);
                self.insert(body);
            }
        }
    }

    // pub fn collect_rectangles(&self) -> Vec<&'a Rectangle> {
    // }

}

#[cfg(test)]
mod tests {
    use super::{Body, Point, QuadTree, Rectangle};
    use std::rc::Rc;

    #[test]
    pub fn create_and_insert() {
        let region = Rectangle {
            corner: Point { x: 0.0, y: 0.0 },
            w: 10.0,
            h: 10.0,
        };

        let bodies = vec![
            Rc::new(Body {
                position: Point { x: 1.0, y: 1.0 },
                total_mass: 0.0,
            }),
            Rc::new(Body {
                position: Point { x: 8.0, y: 2.0 },
                total_mass: 0.0,
            }),
            Rc::new(Body {
                position: Point { x: 8.0, y: 4.0 },
                total_mass: 0.0,
            }),
            Rc::new(Body {
                position: Point { x: 8.0, y: 8.0 },
                total_mass: 0.0,
            }),
        ];

        let mut tree = QuadTree::new(region);

        bodies.iter().for_each(|b| tree.insert(Rc::clone(b)));

        println!("{:#?}", tree)
    }
}

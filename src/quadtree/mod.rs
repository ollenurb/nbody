#![allow(dead_code)]

mod body;
pub use body::Body;

mod rectangle;
pub use rectangle::Rectangle;

mod point;
pub use point::Point;

use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct QuadTree {
    boundary: Rectangle,
    node: Node,
}

// A node can be either:
//     * Empty
//     * External
//     * Internal
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
    use crate::quadtree::{Point, QuadTree, Rectangle, Body};
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

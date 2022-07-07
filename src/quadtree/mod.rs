#![allow(dead_code)]

mod body;
pub use body::Body;

mod bound;
pub use bound::Bound;

mod vec_2d;
pub use vec_2d::Vec2D;

#[derive(Debug, Clone)]
pub struct QuadTree {
    boundary: Bound,
    node: Node,
}

// A node can be either:
//
// * Empty
// * External
// * Internal
//
#[derive(Debug, Clone)]
enum Node {
    Empty,

    // Body is stored as a mutable reference
    External(Body),

    Internal {
        // (mx, my): Center of mass
        cluster: Body,
        nw: Box<QuadTree>,
        ne: Box<QuadTree>,
        sw: Box<QuadTree>,
        se: Box<QuadTree>,
    },
}

impl QuadTree {
    // Splits the space given by `Bound` into 4 SubSpaces and then returns the SubTree with such
    // generated subspaces as childrens
    fn split_space(bound: &Bound) -> Node {
        let childs = bound.subdivide();
        Node::Internal {
            cluster: Default::default(),
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
        }
    }

    pub fn new(w: u32, h: u32) -> Self {
        let whole_space = Bound {
            x: 0.0,
            y: 0.0,
            w: w as f64,
            h: h as f64,
        };

        QuadTree {
            boundary: whole_space,
            node: Node::Empty,
        }
    }

    pub fn insert(&mut self, body: Body) {
        match self.node {
            // If the node doesn't contain any node, then put the new body here
            Node::Empty => self.node = Node::External(body),

            // if the node is an internal node, update mass and center of mass, then insert it
            // recursively into the appropriate quadrant
            Node::Internal { ref mut cluster, ref mut nw, ref mut ne, ref mut sw, ref mut se } => {
                cluster.update_foces(&body);
                if nw.boundary.contains(&body) {
                    nw.insert(body);
                } else if ne.boundary.contains(&body) {
                    ne.insert(body);
                } else if sw.boundary.contains(&body) {
                    sw.insert(body);
                } else if se.boundary.contains(&body) {
                    se.insert(body);
                } else {
                    panic!("A body cannot be inserted in any boundary")
                }
            }

            // If node x is an external node, say containing a body named c, then there are two
            // bodies b and c in the same region. Subdivide the region further by creating four
            // children. Then, recursively insert both b and c into the appropriate quadrant(s).
            // Since b and c may still end up in the same quadrant, there may be several
            // subdivisions during a single insertion. Finally, update the center-of-mass and total
            // mass of x.
            Node::External(c) => {
                self.node = QuadTree::split_space(&self.boundary);
                self.insert(c);
                self.insert(body);
            }
        }
    }

    // pub fn collect_rectangles(&self) -> Vec<&'a Rectangle> {
    // }
}

#[cfg(test)]
mod tests {
    use crate::{
        consts::*,
        quadtree::{Body, Vec2D, QuadTree},
    };

    #[test]
    pub fn create_and_insert() {
        let bodies = vec![
            Body {
                position: Vec2D { x: 1.0, y: 1.0 },
                mass: 0.0,
            },
            Body {
                position: Vec2D { x: 8.0, y: 2.0 },
                mass: 0.0,
            },
            Body {
                position: Vec2D { x: 8.0, y: 4.0 },
                mass: 0.0,
            },
            Body {
                position: Vec2D { x: 8.0, y: 8.0 },
                mass: 0.0,
            },
        ];

        let mut tree = QuadTree::new(WIDTH, HEIGHT);

        bodies.iter().for_each(|b| tree.insert(b.clone()));

        println!("{:#?}", tree)
    }
}

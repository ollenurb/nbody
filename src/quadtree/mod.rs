#![allow(dead_code)]

mod body;
pub use body::Body;

mod bound;
pub use bound::Bound;

mod vec_2d;
pub use vec_2d::Vec2D;

use crate::consts::THETA;

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

    // Compute the exerted force with respect to another Body
    pub fn update_cluster(a: &mut Body, b: &Body) {
        let new_mass = a.mass + b.mass;
        a.position.x = ((a.position.x * a.mass) + (b.position.x * b.mass)) / new_mass;
        a.position.y = ((a.position.y * a.mass) + (b.position.y * b.mass)) / new_mass;
        a.mass = new_mass;
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
            Node::Internal {
                ref mut cluster,
                ref mut nw,
                ref mut ne,
                ref mut sw,
                ref mut se,
            } => {
                QuadTree::update_cluster(cluster, &body);
                if nw.boundary.contains(&body) {
                    nw.insert(body);
                } else if ne.boundary.contains(&body) {
                    ne.insert(body);
                } else if sw.boundary.contains(&body) {
                    sw.insert(body);
                } else if se.boundary.contains(&body) {
                    se.insert(body);
                } else {
                    // panic!("A body cannot be inserted in any boundary")
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

    pub fn compute_force(&self, body: &mut Body) {
        match &self.node {
            // An empty Node doesn't exert any force
            Node::Empty => (),
            // A cluster of nodes can exert force iff it's distant enough
            Node::External(a) => {
                if a.position == body.position { return }
                body.update_force(a)
            },
            Node::Internal {
                ref cluster,
                nw,
                ne,
                sw,
                se,
            } => {
                let dist = self.boundary.w / body.dist(cluster);

                if dist < THETA {
                    body.update_force(cluster);
                } else {
                    nw.compute_force(body);
                    ne.compute_force(body);
                    sw.compute_force(body);
                    se.compute_force(body);
                }

            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        consts::*,
        quadtree::{Body, QuadTree, Vec2D},
    };

    use std::time::Instant;

    #[test]
    pub fn create_and_insert() {
        let bodies = vec![
            Body {
                position: Vec2D { x: 1.0, y: 1.0 },
                mass: 1.0,
                velocity: Vec2D { x: 1.0, y: 1.0 },
                force: Default::default(),
            },
            Body {
                position: Vec2D { x: 8.0, y: 2.0 },
                mass: 1.0,
                velocity: Vec2D { x: 1.0, y: 1.0 },
                force: Default::default(),
            },
            Body {
                position: Vec2D { x: 8.0, y: 4.0 },
                mass: 1.0,
                velocity: Vec2D { x: 1.0, y: 1.0 },
                force: Default::default(),
            },
            Body {
                position: Vec2D { x: 8.0, y: 8.0 },
                mass: 1.0,
                velocity: Vec2D { x: 1.0, y: 1.0 },
                force: Default::default(),
            },
        ];

        let mut tree = QuadTree::new(WIDTH, HEIGHT);

        bodies.iter().for_each(|b| tree.insert(b.clone()));
        let mut a = bodies[0];
        tree.compute_force(&mut a);
        println!("(a) after computing force using BHT: {:#?}", a);
    }

    #[test]
    pub fn compute_force_must_not_be_nan() {
        let mut a = Body {
            position: Vec2D { x: 8.0, y: 4.0 },
            mass: 1.0,
            velocity: Vec2D { x: 1.0, y: 1.0 },
            force: Default::default(),
        };

        let b = Body {
            position: Vec2D { x: 8.0, y: 8.0 },
            mass: 1.0,
            velocity: Vec2D { x: 1.0, y: 1.0 },
            force: Default::default(),
        };


        a.update_force(&b);
        println!("(a) after computing force exerted by (b): {:#?}", a);
    }

    #[test]
    fn bench() {
        let mut bodies = Vec::new();
        let items = 10000;
        let (w, h) = (350, 600);

        for _ in 0..items {
            bodies.push(Body::from_random(w as f64, h as f64, 1.0));
        }

        let start = Instant::now();
        let mut tree = QuadTree::new(w, h);
        bodies.iter().for_each(|b| tree.insert(*b));
        let duration = start.elapsed();
        println!("Inserted {} items in: {:?}", items, duration);

        let start = Instant::now();
        for body in bodies.iter_mut() {
            tree.compute_force(body);
        }

        let duration = start.elapsed();
        println!("Computed forces of {} items in: {:?}", items, duration);
    }
}

#![allow(dead_code)]

mod body;
pub use body::Body;

mod bound;
pub use bound::Bound;

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
        let m_tot = a.mass + b.mass;
        a.position = (a.position * a.mass + b.position * b.mass) / m_tot;
        a.mass = m_tot;
    }

    pub fn new(w: f64, h: f64) -> Self {
        let whole_space = Bound {
            x: 0.0,
            y: 0.0,
            w,
            h,
        };

        QuadTree {
            boundary: whole_space,
            node: Node::Empty,
        }
    }

    /// Iterative insertion method
    pub fn insert(&mut self, body: Body) {
        let mut nodes_stack: Vec<&mut QuadTree> = Vec::new();
        nodes_stack.push(self);
        while !nodes_stack.is_empty() {
            let current = nodes_stack.pop().expect("Cannot pop an empty stack");
            match current.node {
                Node::Empty => current.node = Node::External(body),

                Node::Internal {
                    ref mut cluster,
                    ref mut nw,
                    ref mut ne,
                    ref mut sw,
                    ref mut se,
                } => {
                    QuadTree::update_cluster(cluster, &body);
                    if nw.boundary.contains(&body) {
                        nodes_stack.push(nw);
                    } else if ne.boundary.contains(&body) {
                        nodes_stack.push(ne);
                    } else if sw.boundary.contains(&body) {
                        nodes_stack.push(sw);
                    } else if se.boundary.contains(&body) {
                        nodes_stack.push(se);
                    }
                }

                Node::External(cluster) => {
                    current.node = QuadTree::split_space(&current.boundary);
                    current.insert(cluster);
                    nodes_stack.push(current);
                }
            }
        }
    }

    /// Recursive insertion method.
    /// It is more elegant but it can overflow the stack with huge number of bodies, leading to an
    /// uncontrolled crash of the entire program
    pub fn insert_rec(&mut self, body: Body) {
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
                    nw.insert_rec(body);
                } else if ne.boundary.contains(&body) {
                    ne.insert_rec(body);
                } else if sw.boundary.contains(&body) {
                    sw.insert_rec(body);
                } else if se.boundary.contains(&body) {
                    se.insert_rec(body);
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
                self.insert_rec(c);
                self.insert_rec(body);
            }
        }
    }

    pub fn get_rectangles(&self) -> Vec<Bound> {
        let mut bounds: Vec<Bound> = Vec::new();
        let mut stack: Vec<QuadTree> = Vec::new();
        while !stack.is_empty() {
            let cur = stack.pop().unwrap();
            // push the boundary inside
            bounds.push(cur.boundary);
            // Advance the recursion
            match stack.pop().unwrap().node {
                Node::Internal { nw, ne, sw, se, .. } => {
                    stack.push(*nw);
                    stack.push(*ne);
                    stack.push(*sw);
                    stack.push(*se);
                }
                _ => (),
            }
        }
        bounds
    }

    pub fn compute_force_rec(&self, body: &mut Body) {
        match &self.node {
            // An empty Node doesn't exert any force
            Node::Empty => (),
            Node::External(a) => {
                if a.position != body.position {
                    body.update_force(a)
                }
            }
            Node::Internal {
                ref cluster,
                nw,
                ne,
                sw,
                se,
            } => {
                // A cluster of nodes can exert force iff it's distant enough
                let dist = self.boundary.w / body.dist(cluster);

                if dist < THETA {
                    body.update_force(cluster);
                } else {
                    nw.compute_force_rec(body);
                    ne.compute_force_rec(body);
                    sw.compute_force_rec(body);
                    se.compute_force_rec(body);
                }
            }
        }
    }

    pub fn close_bodies<'a>(&'a self, body: Body) -> QuadTreeIterator {
        QuadTreeIterator {
            tree: self,
            body,
            theta: THETA,
            stack: vec![self],
        }
    }

}

pub struct QuadTreeIterator<'a> {
    tree: &'a QuadTree,
    body: Body,
    theta: f64,
    stack: Vec<&'a QuadTree>
}

impl<'a> Iterator for QuadTreeIterator<'a> {
    type Item = &'a Body;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let current = self.stack.pop()?;
            match &current.node {
                Node::External(b) if b.position != self.body.position => {
                   return Some(b);
                },
                Node::Internal { cluster, nw, ne, sw, se } => {
                    let dist = current.boundary.w / self.body.dist(&cluster);
                    if dist < self.theta { return Some(cluster) }
                    else {
                        self.stack.push(nw);
                        self.stack.push(ne);
                        self.stack.push(sw);
                        self.stack.push(se);
                    }
                },
                _ => ()
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::quadtree::{Body, QuadTree};

    use std::time::Instant;

    #[test]
    pub fn create_and_insert() {
        let bounds = (100.0, 100.0);

        let mut bodies = vec![
            Body::new(54.0, 26.0, 1.0, 4.0, 15.7),
            Body::new(47.0, 21.0, -1.0, 0.0, 5.6),
            Body::new(8.0, 97.0, -1.0, -1.0, 62.5),
            Body::new(51.7, 52.3, 0.0, 0.0, 1e3),
            Body::new(64.0, 72.0, 0.0, 0.0, 1.0),
        ];

        let mut tree = QuadTree::new(bounds.0, bounds.1);

        bodies.iter().for_each(|b| tree.insert_rec(b.clone()));
        bodies.iter_mut().for_each(|b| tree.compute_force_rec(b));
        println!("{:#?}", bodies);
        println!("{:#?}", tree);
    }

    #[test]
    pub fn compute_force_must_not_be_nan() {
        let mut a = Body::new(8.0, 4.0, 1.0, 1.0, 1.0);
        let b = Body::new(8.0, 8.0, 1.0, 1.0, 1.0);

        a.update_force(&b);
        println!("(a) after computing force exerted by (b): {:#?}", a);
    }

    #[test]
    fn bench() {
        let mut bodies = Vec::new();
        let items = 10000;
        let (w, h) = (350.0, 600.0);

        for i in 0..items {
            bodies.push(Body::new(i as f64 / w, i as f64 / h, 1.0, 1.0, 1.0));
        }

        let start = Instant::now();
        let mut tree = QuadTree::new(w, h);
        bodies.iter().for_each(|b| tree.insert_rec(*b));
        let duration = start.elapsed();
        println!("Inserted {} items in: {:?}", items, duration);

        let start = Instant::now();
        for body in bodies.iter_mut() {
            tree.compute_force_rec(body);
        }

        let duration = start.elapsed();
        println!("Computed forces of {} items in: {:?}", items, duration);
    }

    #[test]
    fn test_distance_between_bodies() {
        let a = Body::new(-7.0, -4.0, 0.0, 0.0, 0.0);
        let b = Body::new(17.0, 6.5, 0.0, 0.0, 0.0);
        println!("distance between a and b is {}", a.dist(&b));
    }
}

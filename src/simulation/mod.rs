use super::consts::*;
use super::quadtree::*;

use super::quadtree::{QuadTree, Point};

use std::rc::Rc;

pub struct Simulation {
    bodies: Vec<Rc<Body>>,
    tree: QuadTree,
}

impl Simulation {
    pub fn new() -> Self {
        let tree = QuadTree::new(WIDTH, HEIGHT);

        Simulation {
            bodies: Vec::new(),
            tree,
        }
    }

    pub fn test_init(&mut self) {
        let bodies = vec![
            Body {
                position: Point { x: 247, y: 67 },
                mass: 0.0,
            },
            Body {
                position: Point { x: 229, y: 181 },
                mass: 0.0,
            },
            Body {
                position: Point { x: 126, y: 112 },
                mass: 0.0,
            },
            Body {
                position: Point { x: 201, y: 205 },
                mass: 0.0,
            },
        ];

        bodies.iter().for_each(|b| self.add_body(*b))
    }

    pub fn add_body(&mut self, body: Body) {
        let rc_body = Rc::new(body);
        self.bodies.push(rc_body);
    }

    /// Update the `World` internal state; bounce the box around the screen.
    pub fn update(&mut self) {

        self.bodies
            .iter()
            .map(Rc::clone)
            .for_each(|b| self.tree.insert(b))
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            // We need to compute the indexes manually
            let x = i as u32 % WIDTH;
            let y = i as u32 / WIDTH;

            let rgba = if self.bodies.iter()
                .any(|b| x <= b.position.x + 5 && x >= b.position.x && y >= b.position.y && y <= b.position.y + 5) {
                [0x48, 0xb2, 0xe8, 0xff]
            } else {
                [0x5e, 0x48, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}

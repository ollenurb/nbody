use super::consts::*;
use super::quadtree::*;

use super::quadtree::{QuadTree, Vec2D};

pub struct Simulation {
    bodies: Vec<Body>,
    tree: QuadTree,
}

impl Simulation {
    pub fn new() -> Self {
        Simulation {
            bodies: Vec::new(),
            tree: QuadTree::new(WIDTH, HEIGHT),
        }
    }

    pub fn test_init(&mut self) {
        let bodies = vec![
            Body {
                position: Vec2D { x: 161.0, y: 123.5 },
                mass: 1e14,
                velocity: Vec2D { x: 0.0, y: 0.0 },
                force: Default::default(),
            },
            Body {
                position: Vec2D { x: 229.0, y: 181.0 },
                mass: 5.0,
                velocity: Vec2D { x: 0.0, y: 0.0 },
                force: Default::default(),
            },
            Body {
                position: Vec2D { x: 126.0, y: 112.0 },
                mass: 4.0,
                velocity: Vec2D { x: 0.0, y: 0.0 },
                force: Default::default(),
            },
            Body {
                position: Vec2D { x: 201.0, y: 205.0 },
                mass: 86.0,
                velocity: Vec2D { x: 0.0, y: 0.0 },
                force: Default::default(),
            },
        ];

        bodies.iter().for_each(|b| self.bodies.push(b.clone()))
    }

    /// Update the world internal state by recomputing forces for each body
    /// We use the Barnes Hut Algorithm here
    pub fn update(&mut self) {
        // Create a tree from the bodies
        self.tree = QuadTree::new(WIDTH, HEIGHT);

        self.bodies.iter_mut().for_each(|b| {
            b.reset_force();
            self.tree.insert(*b)
        });

        self.bodies.iter_mut().for_each(|b| {
            self.tree.compute_force(b);
            b.update_position();
        });

        // println!("{:?}", self.bodies[1].velocity);
    }

    // TODO: Refactor draw pipeline
    // Draw a rectangle
    pub fn draw_rect(&self, frame: &mut [u8], s: (u32, u32), e: (u32, u32)) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            // We need to compute the indexes manually
            let x = i as u32 % WIDTH;
            let y = i as u32 / WIDTH;

            let top = (y == s.1) && (x >= s.0) && (x <= e.0);
            let bottom = (y == e.1) && (x >= s.0) && (x <= e.0);
            let left = (x == s.0) && (y >= s.1) && (y <= e.1);
            let right = (x == e.0) && (y >= s.1) && (y <= e.1);

            let rgba = if top || bottom ||  left || right {
                [0x48, 0xb2, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            // We need to compute the indexes manually
            let x = i as u32 % WIDTH;
            let y = i as u32 / WIDTH;

            let rgba = if self.bodies.iter().any(|b| {
                let ix: u32 = b.position.x as u32;
                let iy: u32 = b.position.y as u32;
                // x <= ix + 5 && x >= ix && y >= iy && y <= iy + 5
                x <= ix && x >= ix && y >= iy && y <= iy
            }) {
                [0xff, 0xff, 0xff, 0xff]
            } else {
                [0x00, 0x00, 0x00, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}

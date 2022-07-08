use super::consts::*;
use super::quadtree::*;

use super::quadtree::{QuadTree, Vec2D};

pub struct Simulation {
    bodies: Vec<Body>,
}

impl Simulation {
    pub fn new() -> Self {
        Simulation {
            bodies: Vec::new(),
        }
    }

    pub fn test_init(&mut self) {
        let bodies = vec![
            Body {
                position: Vec2D { x: 160.0, y: 120.0 },
                mass: 100.0,
                velocity: Vec2D { x: 0.0, y: 0.0 },
                force: Default::default(),
            },
            Body {
                position: Vec2D { x: 229.0, y: 181.0 },
                mass: 10.0,
                velocity: Vec2D { x: -0.1, y: -0.1 },
                force: Default::default(),
            },
            Body {
                position: Vec2D { x: 126.0, y: 112.0 },
                mass: 2.0,
                velocity: Vec2D { x: 0.1, y: 0.1 },
                force: Default::default(),
            },
            Body {
                position: Vec2D { x: 201.0, y: 205.0 },
                mass: 3.0,
                velocity: Vec2D { x: -0.1, y: -0.1 },
                force: Default::default(),
            },
        ];

        bodies.iter().for_each(|b| self.bodies.push(b.clone()))
    }

    /// Update the world internal state by recomputing forces for each body
    /// We use the Barnes Hut Algorithm here
    pub fn update(&mut self) {

        // Create a tree from the bodies
        let mut tree = QuadTree::new(WIDTH, HEIGHT);
        self.bodies
            .iter_mut()
            .for_each(|b| {
                b.reset_force();
                tree.insert(*b)
            });

        self.bodies.iter_mut().for_each(|b| {
            tree.compute_force(b);
            b.update_position();
        });
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
                .any(|b| {
                    let ix: u32 = b.position.x as u32;
                    let iy: u32 = b.position.y as u32;
                     x <= ix + 5 && x >= ix && y >= iy && y <= iy + 5
                }) {
                [0x48, 0xb2, 0xe8, 0xff]
            } else {
                [0x5e, 0x48, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}

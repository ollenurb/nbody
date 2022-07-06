use crate::consts::*;
use crate::quadtree::*;

pub struct Simulation {
    bodies: Vec<Body>,
    tree: QuadTree,
}

impl Simulation {
    pub fn new(rect: Rectangle) -> Self {
        let bodies = vec![
            Body {
                position: Point { x: 1, y: 1 },
                total_mass: 0.0,
            },
            Body {
                position: Point { x: 8, y: 2 },
                total_mass: 0.0,
            },
            Body {
                position: Point { x: 8, y: 2 },
                total_mass: 0.0,
            },
            Body {
                position: Point { x: 8, y: 8 },
                total_mass: 0.0,
            },
        ];

        let mut tree = QuadTree::new(rect);

        // bodies.iter().for_each();

        Simulation {
            bodies,
            tree,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    pub fn update(&mut self) {
        // DO nothing
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            // We need to compute the indexes manually
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let rgba = if self.bodies.iter().any(|&b| b.position.x == x && b.position.y == y) {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}

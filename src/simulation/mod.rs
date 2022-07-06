use crate::consts::*;
use crate::quadtree::*;

pub struct Simulation {
    bodies: Vec<Body>,
    tree: QuadTree,
}

impl Simulation {
    pub fn new(rect: Rectangle) -> Self {
        // TODO: TO REMOVE
        let bodies = vec![
            Body {
                position: Point { x: 247.0, y: 67.0 },
                total_mass: 0.0,
            },
            Body {
                position: Point { x: 229.0, y: 181.0 },
                total_mass: 0.0,
            },
            Body {
                position: Point { x: 126.0, y: 112.0 },
                total_mass: 0.0,
            },
            Body {
                position: Point { x: 201.0, y: 205.0 },
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
            let x = (i as f64) % (WIDTH as f64);
            let y = (i as f64) / (WIDTH as f64);

            let rgba = if self.bodies.iter()
                .any(|&b| x <= b.position.x + 5.0 && x >= b.position.x && y >= b.position.y && y <= b.position.y + 5.0) {
                [0x48, 0xb2, 0xe8, 0xff]
            } else {
                [0x5e, 0x48, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}

use std::fs::File;
use std::io::{Result, BufReader, BufRead, self};

use super::consts::*;
use super::quadtree::*;

use super::quadtree::{QuadTree, Vec2D};

mod renderizable_body;

// Structure that sepresents both the simulation state and the simulation parameters
#[derive(Debug)]
pub struct Simulation {
    bodies: Vec<Body>,
    min_max: (f64, f64),
}

impl Simulation {
    pub fn load_from_file(path: &str) -> Result<Simulation> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut simulation = Simulation {
            bodies: Vec::new(),
            min_max: (0.0, 0.0),
        };

        for line in reader.lines() {
            let nums: Vec<f64> = line?
                .split(' ')
                .map(|s| s.parse().expect("Cannot convert str to f64"))
                .collect();

            match nums.len() {
                1 => simulation.min_max = (-nums[0], nums[0]),
                5 => {
                    let body = Body::new(
                        Vec2D {
                            x: nums[0],
                            y: nums[1],
                        },
                        Vec2D {
                            x: nums[2],
                            y: nums[3],
                        },
                        nums[4],
                    );
                    simulation.bodies.push(body);
                }
                _ => {
                    return Result::Err(io::Error::new(io::ErrorKind::Other, "Unexpected file format"));
                }
            }
        }
        Ok(simulation)
    }

    pub fn test_init(&mut self) {
        let bodies = vec![
            Body {
                position: Vec2D { x: 161.0, y: 123.5 },
                mass: 1e15,
                velocity: Vec2D { x: 0.0, y: 0.0 },
                force: Default::default(),
            },
            Body {
                position: Vec2D { x: 229.0, y: 181.0 },
                mass: 105.0,
                velocity: Vec2D { x: -0.1, y: 0.1 },
                force: Default::default(),
            },
            Body {
                position: Vec2D { x: 126.0, y: 112.0 },
                mass: 184.0,
                velocity: Vec2D { x: 0.0, y: 1000.0 },
                force: Default::default(),
            },
            Body {
                position: Vec2D { x: 201.0, y: 205.0 },
                mass: 86.0,
                velocity: Vec2D { x: 0.0, y: 0.0 },
                force: Default::default(),
            },
        ];

        bodies.iter().for_each(|b| self.bodies.push(*b))
    }

    /// Update the world internal state by recomputing forces for each body
    /// We use the Barnes Hut Algorithm here
    pub fn update(&mut self) {
        // Create a tree from the bodies
        let mut tree = QuadTree::new(WIDTH, HEIGHT);

        self.bodies.iter_mut().for_each(|b| {
            b.reset_force();
            tree.insert(*b)
        });

        self.bodies.iter_mut().for_each(|b| {
            tree.compute_force(b);
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

            let rgba = if top || bottom || left || right {
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



#[cfg(test)]
mod test {
    use super::Simulation;


    #[test]
    pub fn load_from_file() {
        let sim = Simulation::load_from_file("galaxy1.txt").unwrap();
        println!("{:#?}", sim);
    }

}

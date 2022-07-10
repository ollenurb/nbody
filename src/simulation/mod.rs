use std::fs::File;
use std::io::{self, BufRead, BufReader, Result};

use self::renderizable_body::RenderizableBody;

use super::consts::*;
use super::quadtree::*;
use crate::util::*;

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
                    return Result::Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Unexpected file format",
                    ));
                }
            }
        }
        Ok(simulation)
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
        let frame_u32 =
            unsafe { core::slice::from_raw_parts_mut(frame.as_ptr() as *mut u32, frame.len() / 4) };

        // Clear buffer
        for pixel in frame_u32.iter_mut() {
            *pixel = 0xff000000;
        }

        // First, transform the bodies into renderizable objects
        self.bodies
            .iter()
            .map(|b| RenderizableBody {
                x: map_range(
                    b.position.x,
                    self.min_max.0,
                    self.min_max.1,
                    0.0,
                    WIDTH as f64,
                ) as u32,
                y: map_range(
                    b.position.y,
                    self.min_max.0,
                    self.min_max.1,
                    0.0,
                    HEIGHT as f64,
                ) as u32,
            })
            .for_each(|body| {
                let i = ((body.y * WIDTH) + body.x) as usize;
                if i < frame_u32.len() {
                    frame_u32[i] = 0xffffffff;
                }
            })
    }
}

#[cfg(test)]
mod test {
    use std::time::Instant;

    use crate::{consts::HEIGHT, consts::WIDTH, quadtree::QuadTree};

    use super::Simulation;

    #[test]
    pub fn load_from_file_then_generate_tree() {
        let mut sim = Simulation::load_from_file("galaxy1.txt").unwrap();

        let start = Instant::now();
        let mut tree = QuadTree::new(WIDTH, HEIGHT);
        sim.bodies.iter().for_each(|b| tree.insert(*b));
        sim.bodies.iter_mut().for_each(|b| tree.compute_force(b));
        let elapsed = start.elapsed();
        println!("Computed {} bodies in {:?}", sim.bodies.len(), elapsed);
    }
}

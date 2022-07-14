use std::fs::File;
use std::io::{self, BufRead, BufReader, Result};

use super::consts::*;
use super::quadtree::*;
use crate::util::*;

use super::quadtree::QuadTree;

// Structure that sepresents both the simulation state and the simulation parameters
#[derive(Debug)]
pub struct Simulation {
    bodies: Vec<Body>,
    min_val: f64,
    max_val: f64,
}

impl Simulation {
    pub fn load_from_file(path: &str) -> Result<Simulation> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut bodies: Vec<Body> = Vec::new();
        let mut min_max = (0.0, 0.0);

        for line in reader.lines() {
            let nums: Vec<f64> = line?
                .split(' ')
                .map(|s| s.parse().expect("Cannot convert str to f64"))
                .collect();

            match nums.len() {
                1 => min_max = (-nums[0], nums[0]),
                5 => {
                    let body = Body::new(nums[0], nums[1], nums[2], nums[3], nums[4]);
                    bodies.push(body);
                }
                _ => {
                    return Result::Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Unexpected file format",
                    ));
                }
            }
        }
        Ok(Simulation {
            bodies,
            min_val: min_max.0,
            max_val: min_max.1,
            // tree: QuadTree::new(min_max.1, min_max.1),
        })
    }

    /// Update the world internal state by recomputing forces for each body
    /// We use the Barnes Hut Algorithm here
    pub fn update(&mut self) {
        // First, filter out bodies that are going out of simulation boundaries
        self.bodies.retain(|b| {
            b.position.x >= self.min_val
                && b.position.x <= self.max_val
                && b.position.y >= self.min_val
                && b.position.y <= self.max_val
        });

        // Create a tree from the bodies
        let mut tree = QuadTree::new(self.max_val, self.max_val);

        self.bodies.iter_mut().for_each(|b| {
            b.reset_force();
            tree.insert(*b)
        });

        self.bodies.iter_mut().for_each(|b| {
            tree.close_bodies(b.clone())
                .into_iter()
                .for_each(|cb| b.update_force(cb));
            b.update_position(1e-1);
        });
    }

    /// Draw the `World` state to the frame buffer.
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
            .map(|b| affine_transform(b.position, self.min_val, self.max_val))
            .map(|b| (b.x as u32, b.y as u32))
            .for_each(|body| {
                let i = ((body.1 * WIDTH) + body.0) as usize;
                if i < frame_u32.len() {
                    frame_u32[i] = 0xffffffff;
                }
            })
    }
}

#[cfg(test)]
mod test {
    use std::time::Instant;

    use crate::quadtree::QuadTree;

    use super::Simulation;

    #[test]
    pub fn load_from_file_then_generate_tree() {
        let mut sim = Simulation::load_from_file("galaxy10k.txt").unwrap();

        let start = Instant::now();
        let mut tree = QuadTree::new(sim.max_val, sim.max_val);
        sim.bodies.iter().for_each(|b| tree.insert(*b));
        sim.bodies
            .iter_mut()
            .for_each(|b| tree.compute_force_rec(b));
        let elapsed = start.elapsed();
        println!("Computed {} bodies in {:?}", sim.bodies.len(), elapsed);
    }
}

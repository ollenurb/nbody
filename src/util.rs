use ultraviolet::{DVec2, DMat2};

use crate::consts::{WIDTH, HEIGHT};

pub fn affine_transform(pos: DVec2, min: f64, max: f64) -> DVec2 {
    let sx = WIDTH as f64 / (max - min);
    let sy = HEIGHT as f64 / (max - min);

    let transform = DMat2::new(DVec2::new(sx, 0.0), DVec2::new(0.0, sy));
    let offset = DVec2::new(WIDTH as f64 / 2.0, HEIGHT as f64 / 2.0);

    (transform * pos) + offset
}

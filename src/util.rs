
pub fn map_range(v: f64, old_min: f64, old_max: f64, new_min: f64, new_max: f64) -> f64 {
    (((v - old_min) * (new_max - new_min)) / (old_max - old_min)) + new_min
}


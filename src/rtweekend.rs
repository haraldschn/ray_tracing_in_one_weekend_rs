use rand::Rng;
use std::f64::consts::PI;

#[inline(always)]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[inline(always)]
pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

#[inline(always)]
pub fn random_double_inv(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

use rand::Rng;

pub use std::f64::consts::PI;
pub use std::f64::INFINITY;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    rand::thread_rng().gen()
}

pub fn random_int_range(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min..=max)
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
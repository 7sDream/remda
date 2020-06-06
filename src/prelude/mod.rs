mod color;
mod ray;
mod vec3;

pub use {
    color::Color,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub use std::f64::consts::PI;

pub fn d2r(d: f64) -> f64 {
    d * PI / 180.0
}

pub fn r2d(r: f64) -> f64 {
    r / PI * 180.0
}

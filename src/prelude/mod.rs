mod color;
mod ray;
mod vec3;

use {
    rand::Rng,
    std::ops::{Range, RangeInclusive},
};

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

pub fn clamp(x: f64, range: RangeInclusive<f64>) -> f64 {
    if range.start().gt(&x) {
        *range.start()
    } else if range.end().lt(&x) {
        *range.end()
    } else {
        x
    }
}

pub struct Random();

impl Random {
    pub fn normal() -> f64 {
        rand::thread_rng().gen_range(0.0, 1.0)
    }
    pub fn range(r: Range<f64>) -> f64 {
        rand::thread_rng().gen_range(r.start, r.end)
    }
}

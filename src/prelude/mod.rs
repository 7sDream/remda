mod color;
mod random;
mod ray;
mod vec3;

use std::ops::{Bound, RangeBounds};

pub use {
    color::Color,
    random::{Random, SeedRandom},
    ray::Ray,
    std::f64::{consts::PI, INFINITY},
    vec3::{Point3, Vec3},
};

#[must_use]
pub fn d2r(d: f64) -> f64 {
    d * PI / 180.0
}

#[must_use]
pub fn r2d(r: f64) -> f64 {
    r / PI * 180.0
}

#[must_use]
pub fn clamp<R: RangeBounds<f64>>(x: f64, range: R) -> f64 {
    let start = match range.start_bound() {
        Bound::Included(&x) | Bound::Excluded(&x) => x,
        _ => std::f64::NEG_INFINITY,
    };
    let end = match range.end_bound() {
        Bound::Included(&x) | Bound::Excluded(&x) => x,
        _ => std::f64::INFINITY,
    };
    if start > x {
        start
    } else if x > end {
        end
    } else {
        x
    }
}

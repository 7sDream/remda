mod aabb;
mod color;
mod random;
mod ray;
mod vec3;

use std::ops::{Bound, RangeBounds};

pub use {
    aabb::AABB,
    color::Color,
    random::{Random, SeedRandom},
    ray::Ray,
    std::f64::consts::PI,
    vec3::{Point3, Vec3},
};

#[must_use]
pub fn clamp<R: RangeBounds<f64>>(val: f64, range: R) -> f64 {
    let start = match range.start_bound() {
        Bound::Included(&x) | Bound::Excluded(&x) => x,
        _ => std::f64::NEG_INFINITY,
    };
    let end = match range.end_bound() {
        Bound::Included(&x) | Bound::Excluded(&x) => x,
        _ => std::f64::INFINITY,
    };
    if start > val {
        start
    } else if val > end {
        end
    } else {
        val
    }
}

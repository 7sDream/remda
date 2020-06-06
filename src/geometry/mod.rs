mod sphere;

use crate::types::Ray;

pub use sphere::Sphere;

pub trait Geometry {
    fn check_ray_hits(&self, r: &Ray) -> bool;
}

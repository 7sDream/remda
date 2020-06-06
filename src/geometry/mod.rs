mod sphere;

use crate::types::{Point3, Ray, Vec3};

pub use sphere::Sphere;

pub trait Geometry {
    fn hit_by_ray(&self, r: &Ray) -> Option<Point3>;
    fn normal(&self, p: &Point3) -> Vec3;
}

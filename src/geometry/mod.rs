mod sphere;
mod world;

use {crate::prelude::*, std::ops::Range};

pub use sphere::Sphere;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub outside: bool,
}

impl HitRecord {
    pub fn new<G: Geometry + ?Sized>(r: &Ray, object: &G, t: f64) -> Self {
        let point = r.at(t);
        let mut normal = object.normal(&point);
        let outside = r.direction.dot(&normal) < 0.0;
        if !outside {
            normal.reverse();
        }
        Self { point, normal, t, outside }
    }
}

pub trait Geometry {
    fn normal(&self, p: &Point3) -> Vec3;
    fn hit(&self, r: &Ray, limit: Range<f64>) -> Option<HitRecord>;
}

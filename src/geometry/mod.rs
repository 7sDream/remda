mod sphere;

use {
    crate::types::{Point3, Ray, Vec3},
    std::ops::RangeBounds,
};

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
    fn hit_time(&self, r: &Ray) -> Option<(f64, f64)>;
    fn normal(&self, p: &Point3) -> Vec3;

    fn hit<R: RangeBounds<f64>>(&self, r: &Ray, limit: R) -> Option<HitRecord> {
        let (t1, t2) = self.hit_time(r)?;
        let t = if limit.contains(&t1) {
            &t1
        } else if limit.contains(&t2) {
            &t2
        } else {
            return None;
        };
        Some(HitRecord::new(r, self, *t))
    }
}

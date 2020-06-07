mod sphere;
mod world;

use {
    crate::material::Material,
    crate::prelude::*,
    std::{ops::Range, rc::Rc},
};

pub use {sphere::Sphere, world::World};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
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
        let material = object.material();
        Self { point, normal, material, t, outside }
    }
}

pub trait Geometry {
    fn normal(&self, p: &Point3) -> Vec3;
    fn material(&self) -> Rc<dyn Material>;
    fn hit(&self, r: &Ray, limit: Range<f64>) -> Option<HitRecord>;
}

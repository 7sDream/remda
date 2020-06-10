mod hit;
mod sphere;
mod world;

use {
    crate::material::Material,
    crate::prelude::*,
    std::{ops::Range, rc::Rc},
};

pub use {hit::HitRecord, sphere::Sphere, world::World};

pub trait Geometry {
    fn normal(&self, p: &Point3) -> Vec3;
    fn material(&self) -> Rc<dyn Material>;
    fn hit(&self, r: &Ray, limit: Range<f64>) -> Option<HitRecord>;
}

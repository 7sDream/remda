use crate::{geometry::HitRecord, prelude::*};

mod lambertian;

pub use lambertian::Lambertian;

pub struct ScatterRecord {
    pub color: Color,
    pub ray: Ray,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, record: HitRecord) -> Option<ScatterRecord>;
}

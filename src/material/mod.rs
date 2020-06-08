use crate::{geometry::HitRecord, prelude::*};

mod dielectric;
mod lambertian;
mod metal;

pub use {
    dielectric::{Dielectric, Glass},
    lambertian::Lambertian,
    metal::Metal,
};

pub struct ScatterRecord {
    pub color: Color,
    pub ray: Ray,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: HitRecord) -> Option<ScatterRecord>;
}

pub(crate) fn reflect(ray: &Ray, hit: &HitRecord) -> Ray {
    let dir = ray.direction.unit();
    let reflected_dir = &dir - 2.0 * dir.dot(&hit.normal) * &hit.normal;
    Ray::new(hit.point.clone(), reflected_dir)
}

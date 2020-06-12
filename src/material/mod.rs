use crate::{geometry::HitRecord, prelude::*};

mod dielectric;
mod lambertian;
mod metal;

pub use {
    dielectric::{Dielectric, Glass},
    lambertian::Lambertian,
    metal::Metal,
};

#[derive(Debug)]
pub struct ScatterRecord {
    pub color: Color,
    pub ray: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: HitRecord<'_>) -> Option<ScatterRecord>;
}

pub(crate) fn reflect(ray: &Ray, hit: &HitRecord<'_>) -> Ray {
    let dir = ray.direction.unit();
    let reflected_dir = &dir - 2.0 * dir.dot(&hit.normal) * &hit.normal;
    Ray::new(hit.point.clone(), reflected_dir, ray.departure_time)
}

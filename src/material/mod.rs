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

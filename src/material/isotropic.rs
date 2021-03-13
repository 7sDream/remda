use crate::{
    material::{Material, ScatterRecord},
    prelude::*,
};

#[derive(Debug, Clone)]
pub struct Isotropic {
    color: Color,
}

impl Isotropic {
    #[must_use]
    pub const fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self, ray: &Ray, hit: crate::hittable::HitRecord<'_>,
    ) -> Option<super::ScatterRecord> {
        let scattered_ray = Ray::new(hit.point, Vec3::random_in_unit_sphere(), ray.departure_time);
        Some(ScatterRecord {
            ray: scattered_ray,
            color: self.color.clone(),
        })
    }
}

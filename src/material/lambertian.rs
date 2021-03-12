use crate::{
    hittable::HitRecord,
    material::{Material, ScatterRecord},
    prelude::*,
    texture::Texture,
};

#[derive(Debug, Clone)]
pub enum LambertianMathType {
    Approximates,
    True,
    Hemisphere,
}

impl LambertianMathType {
    #[must_use]
    pub fn scatter_ray(&self, ray: &Ray, hit: HitRecord<'_>) -> Ray {
        match self {
            Self::Approximates => Ray::new(
                hit.point,
                hit.normal + Vec3::random_in_unit_sphere(),
                ray.departure_time,
            ),
            Self::True => Ray::new(
                hit.point,
                hit.normal + Vec3::random_unit(),
                ray.departure_time,
            ),
            Self::Hemisphere => Ray::new(
                hit.point,
                Vec3::random_unit_dir(&hit.normal),
                ray.departure_time,
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Lambertian<T: Texture> {
    texture: T,
    math_type: LambertianMathType,
}

impl<T: Texture> Lambertian<T> {
    #[must_use]
    pub fn new(texture: T) -> Self {
        Self {
            texture,
            math_type: LambertianMathType::True,
        }
    }

    #[must_use]
    pub fn math_type(mut self, value: LambertianMathType) -> Self {
        self.math_type = value;
        self
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, ray: &Ray, hit: HitRecord<'_>) -> Option<ScatterRecord> {
        let color = self.texture.color(hit.u, hit.v, &hit.point);
        let new_ray = self.math_type.scatter_ray(ray, hit);
        Some(ScatterRecord {
            color,
            ray: new_ray,
        })
    }
}

use {
    super::{Material, ScatterRecord},
    crate::{geometry::HitRecord, prelude::*},
};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Lambertian {
    color: Color,
    math_type: LambertianMathType,
}

impl Lambertian {
    #[must_use]
    pub const fn new(color: Color) -> Self {
        Self {
            color,
            math_type: LambertianMathType::True,
        }
    }

    #[must_use]
    pub const fn math_type(mut self, value: LambertianMathType) -> Self {
        self.math_type = value;
        self
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: HitRecord<'_>) -> Option<super::ScatterRecord> {
        let new_ray = self.math_type.scatter_ray(ray, hit);
        Some(ScatterRecord {
            color: self.color.clone(),
            ray: new_ray,
        })
    }
}

use {
    super::{Material, ScatterRecord},
    crate::{geometry::HitRecord, prelude::*},
};

#[derive(Debug)]
pub struct Metal {
    color: Color,
    fuzz: f64,
}

impl Metal {
    #[must_use]
    pub const fn new(color: Color) -> Self {
        Self { color, fuzz: 0.0 }
    }

    #[must_use]
    pub fn fuzz(mut self, fuzz: f64) -> Self {
        self.fuzz = clamp(fuzz.abs(), 0.0..=1.0);
        self
    }

    fn reflect(&self, ray: &Ray, hit: &HitRecord<'_>) -> Ray {
        let dir = ray.direction.unit();
        let mut reflected_dir = &dir - 2.0 * dir.dot(&hit.normal) * &hit.normal;
        reflected_dir += self.fuzz * Vec3::random_in_unit_hemisphere(&reflected_dir);
        Ray::new(hit.point.clone(), reflected_dir, ray.departure_time)
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: HitRecord<'_>) -> Option<ScatterRecord> {
        let reflected = self.reflect(ray, &hit);
        if reflected.direction.dot(&hit.normal) > 0.0 {
            Some(ScatterRecord {
                color: self.color.clone(),
                ray: reflected,
            })
        } else {
            None
        }
    }
}

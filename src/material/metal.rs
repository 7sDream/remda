use {
    super::{Material, ScatterRecord},
    crate::{geometry::HitRecord, prelude::*},
};

pub struct Metal {
    color: Color,
}

impl Metal {
    pub fn new(color: Color) -> Self {
        Self { color }
    }

    fn reflect(ray: &Ray, hit: &HitRecord) -> Ray {
        let dir = ray.direction.unit();
        let reflected_dir = &dir - 2.0 * dir.dot(&hit.normal) * &hit.normal;
        Ray::new(hit.point.clone(), reflected_dir)
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: HitRecord) -> Option<ScatterRecord> {
        let reflected = Self::reflect(ray, &hit);
        Some(ScatterRecord {
            color: self.color.clone(),
            ray: reflected,
        })
    }
}

use crate::{
    hittable::HitRecord,
    material::{Material, ScatterRecord},
    prelude::*,
    texture::Texture,
};

#[derive(Debug)]
pub struct Metal<T: Texture> {
    texture: T,
    fuzz: f64,
}

impl<T: Texture> Metal<T> {
    #[must_use]
    pub fn new(texture: T) -> Self {
        Self { texture, fuzz: 0.0 }
    }

    #[must_use]
    pub fn fuzz(mut self, fuzz: f64) -> Self {
        self.fuzz = clamp(fuzz.abs(), 0.0..=1.0);
        self
    }

    fn reflect(&self, ray: &Ray, hit: &HitRecord<'_>) -> Ray {
        let dir = ray.direction.unit();
        let mut reflected_dir = &dir - 2.0 * dir.dot(&hit.normal) * &hit.normal;
        reflected_dir += self.fuzz * Vec3::random_in_unit_sphere();
        Ray::new(hit.point.clone(), reflected_dir, ray.departure_time)
    }
}

impl<T: Texture> Material for Metal<T> {
    fn scatter(&self, ray: &Ray, hit: HitRecord<'_>) -> Option<ScatterRecord> {
        let color = self.texture.color(hit.u, hit.v, &hit.point);
        let reflected = self.reflect(ray, &hit);
        if reflected.direction.dot(&hit.normal) > 0.0 {
            Some(ScatterRecord {
                color,
                ray: reflected,
            })
        } else {
            None
        }
    }
}

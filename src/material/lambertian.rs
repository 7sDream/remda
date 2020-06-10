use {
    super::{Material, ScatterRecord},
    crate::{geometry::HitRecord, prelude::*},
};

#[derive(Debug)]
pub struct Lambertian {
    color: Color,
    use_hemi: bool,
}

impl Lambertian {
    #[must_use]
    pub const fn new(color: Color) -> Self {
        Self {
            color,
            use_hemi: false,
        }
    }

    #[must_use]
    pub const fn hemi(mut self, value: bool) -> Self {
        self.use_hemi = value;
        self
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: HitRecord<'_>) -> Option<super::ScatterRecord> {
        let dir = if self.use_hemi {
            Vec3::random_unit_dir(&hit.normal)
        } else {
            hit.normal + Vec3::random_unit()
        };
        let new_ray = Ray::new(hit.point, dir);
        Some(ScatterRecord {
            color: self.color.clone(),
            ray: new_ray,
        })
    }
}

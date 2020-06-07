use {
    super::{Material, ScatterRecord},
    crate::{geometry::HitRecord, prelude::*},
};

pub struct Lambertian {
    color: Color,
    use_hemi: bool,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Self { color, use_hemi: false }
    }

    pub fn hemi(mut self, value: bool) -> Self {
        self.use_hemi = value;
        self
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, record: HitRecord) -> Option<super::ScatterRecord> {
        let dir = if self.use_hemi {
            Vec3::random_unit_dir(&record.normal)
        } else {
            record.normal + Vec3::random_unit()
        };
        let new_ray = Ray::new(record.point, dir);
        Some(ScatterRecord { color: self.color.clone().into(), ray: new_ray })
    }
}

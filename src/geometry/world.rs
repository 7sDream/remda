use {
    super::{Geometry, HitRecord, AABB, BVH},
    crate::{material::Material, prelude::*},
    std::{
        fmt::{Debug, Formatter},
        ops::Range,
    },
};

pub fn default_background(ray: &Ray) -> Color {
    let unit = ray.direction.unit();
    let t = 0.5 * (unit.y + 1.0);
    Color::newf(1.0, 1.0, 1.0).gradient(&Color::newf(0.5, 0.7, 1.0), t)
}

pub struct World {
    bvh: BVH,
    bg_func: Box<dyn Fn(&Ray) -> Color + Send + Sync>,
}

impl Debug for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("World {}")
    }
}

impl World {
    #[must_use]
    pub fn new(bvh: BVH) -> Self {
        Self {
            bvh,
            bg_func: Box::new(default_background),
        }
    }

    pub fn set_bg<F>(&mut self, f: F)
    where
        F: Fn(&Ray) -> Color + Send + Sync + 'static,
    {
        self.bg_func = Box::new(f);
    }

    #[must_use]
    pub fn background(&self, ray: &Ray) -> Color {
        let f = &self.bg_func;
        f(ray)
    }
}

impl Geometry for World {
    fn normal(&self, _p: &Point3) -> Vec3 {
        unimplemented!("World's normal function should not be called directly")
    }

    fn material(&self) -> &dyn Material {
        unimplemented!("World's material function should not be called directly")
    }

    fn uv(&self, _point: &Point3) -> (f64, f64) {
        unimplemented!("World's uv function should not be called directly")
    }

    fn hit(&self, ray: &Ray, unit_limit: Range<f64>) -> Option<HitRecord<'_>> {
        self.bvh.hit(ray, unit_limit)
    }

    fn bbox(&self, time_limit: Range<f64>) -> Option<AABB> {
        self.bvh.bbox(time_limit)
    }
}

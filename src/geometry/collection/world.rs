use {
    crate::{
        geometry::{
            collection::{GeometryList, BVH},
            Geometry, HitRecord,
        },
        prelude::*,
    },
    std::{
        fmt::{Debug, Formatter},
        ops::Range,
    },
};

#[must_use]
pub fn default_background(ray: &Ray) -> Color {
    let unit = ray.direction.unit();
    let t = 0.5 * (unit.y + 1.0);
    Color::new(1.0, 1.0, 1.0).gradient(&Color::new(0.5, 0.7, 1.0), t)
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
    pub fn new(list: GeometryList, time_range: Range<f64>) -> Self {
        Self {
            bvh: BVH::new(list.into_objects(), time_range),
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
    fn hit(&self, ray: &Ray, unit_limit: Range<f64>) -> Option<HitRecord<'_>> {
        self.bvh.hit(ray, unit_limit)
    }

    fn bbox(&self, time_limit: Range<f64>) -> Option<AABB> {
        self.bvh.bbox(time_limit)
    }
}

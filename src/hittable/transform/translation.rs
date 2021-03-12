use crate::{
    hittable::{HitRecord, Hittable},
    prelude::*,
};

#[derive(Debug, Clone)]
pub struct Translation<T> {
    object: T,
    movement: Vec3,
}

impl<T> Translation<T> {
    pub const fn new(object: T, movement: Vec3) -> Self {
        Self { object, movement }
    }
}

impl<T: Hittable> Hittable for Translation<T> {
    fn hit(&self, ray: &Ray, unit_limit: std::ops::Range<f64>) -> Option<HitRecord<'_>> {
        let moved_ray = Ray::new(
            &ray.origin - &self.movement,
            ray.direction.clone(),
            ray.departure_time,
        );
        self.object.hit(&moved_ray, unit_limit).map(|mut record| {
            record.point += &self.movement;
            record
        })
    }

    fn bbox(&self, time_limit: std::ops::Range<f64>) -> Option<AABB> {
        self.object
            .bbox(time_limit)
            .map(|bbox| AABB::new(bbox.min() + &self.movement, bbox.max() + &self.movement))
    }
}

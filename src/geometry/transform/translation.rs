use crate::{
    geometry::{Geometry, HitRecord},
    prelude::*,
};

#[derive(Debug, Clone)]
pub struct Translation<G> {
    geometry: G,
    movement: Vec3,
}

impl<G> Translation<G> {
    pub const fn new(geometry: G, movement: Vec3) -> Self {
        Self { geometry, movement }
    }
}

impl<G: Geometry> Geometry for Translation<G> {
    fn hit(&self, ray: &Ray, unit_limit: std::ops::Range<f64>) -> Option<HitRecord<'_>> {
        let moved_ray = Ray::new(
            &ray.origin - &self.movement,
            ray.direction.clone(),
            ray.departure_time,
        );
        self.geometry.hit(&moved_ray, unit_limit).map(|mut record| {
            record.point += &self.movement;
            record
        })
    }

    fn bbox(&self, time_limit: std::ops::Range<f64>) -> Option<AABB> {
        self.geometry
            .bbox(time_limit)
            .map(|bbox| AABB::new(bbox.min() + &self.movement, bbox.max() + &self.movement))
    }
}

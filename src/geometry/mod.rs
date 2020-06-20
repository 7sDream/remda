mod aabb;
mod bvh;
mod hit;
mod sphere;
mod world;

use {crate::material::Material, crate::prelude::*, std::ops::Range};

pub use {aabb::AABB, bvh::BVH, hit::HitRecord, sphere::Sphere, world::World};

pub trait Geometry: Send + Sync {
    fn normal(&self, p: &Point3) -> Vec3;
    fn material(&self) -> &dyn Material;
    fn hit(&self, ray: &Ray, limit: Range<f64>) -> Option<HitRecord<'_>>;
    fn bbox(&self, limit: Range<f64>) -> Option<AABB>;
}

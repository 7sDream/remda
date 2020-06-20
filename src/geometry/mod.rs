pub(crate) mod aabb;
pub(crate) mod bvh;
pub(crate) mod hit;
pub mod list;
pub mod sphere;
pub mod world;

use {
    crate::{material::Material, prelude::*},
    std::ops::Range,
};

pub(crate) use {aabb::AABB, bvh::BVH, hit::HitRecord};
pub use {list::GeometryList, sphere::Sphere, world::World};

pub trait Geometry: Send + Sync {
    fn normal(&self, p: &Point3) -> Vec3;
    fn material(&self) -> &dyn Material;
    fn hit(&self, ray: &Ray, unit_limit: Range<f64>) -> Option<HitRecord<'_>>;
    fn bbox(&self, time_limit: Range<f64>) -> Option<AABB>;
}

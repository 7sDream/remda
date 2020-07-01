pub(crate) mod aabb;
pub(crate) mod bvh;
pub(crate) mod hit;
pub(crate) mod list;
pub(crate) mod rect;
pub(crate) mod sphere;
pub(crate) mod world;

use {
    crate::{material::Material, prelude::*},
    std::ops::Range,
};

pub(crate) use {aabb::AABB, bvh::BVH, hit::HitRecord};

pub use {
    list::GeometryList,
    rect::{AARect, AARectGeometry},
    sphere::Sphere,
    world::World,
};

pub trait Geometry: Send + Sync {
    fn normal(&self, p: &Point3) -> Vec3;
    fn material(&self) -> &dyn Material;
    fn uv(&self, point: &Point3) -> (f64, f64);
    fn hit(&self, ray: &Ray, unit_limit: Range<f64>) -> Option<HitRecord<'_>>;
    fn bbox(&self, time_limit: Range<f64>) -> Option<AABB>;
}

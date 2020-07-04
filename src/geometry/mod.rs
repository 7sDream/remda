pub(crate) mod aabb;
pub(crate) mod bvh;
pub(crate) mod carton;
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
    carton::Carton,
    list::GeometryList,
    rect::{AARect, AARectGeometry},
    sphere::Sphere,
    world::{default_background, World},
};

#[allow(unused_variables)]
pub trait Geometry: Send + Sync {
    fn normal(&self, p: &Point3) -> Vec3 {
        unimplemented!(
            "{}'s normal function should not be called directly",
            std::any::type_name::<Self>()
        )
    }
    fn material(&self) -> &dyn Material {
        unimplemented!(
            "{}'s material function should not be called directly",
            std::any::type_name::<Self>()
        )
    }
    fn uv(&self, point: &Point3) -> (f64, f64) {
        unimplemented!(
            "{}'s uv function should not be called directly",
            std::any::type_name::<Self>()
        )
    }
    fn hit(&self, ray: &Ray, unit_limit: Range<f64>) -> Option<HitRecord<'_>>;
    fn bbox(&self, time_limit: Range<f64>) -> Option<AABB>;
}

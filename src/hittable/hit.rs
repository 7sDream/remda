use {
    crate::{material::Material, prelude::*},
    std::{
        fmt::{Debug, Formatter},
        ops::Range,
    },
};

pub struct HitRecord<'m> {
    pub point: Point3,
    pub normal: Vec3,
    pub material: &'m dyn Material,
    pub unit: f64,
    pub u: f64,
    pub v: f64,
    pub outside: bool,
}

impl Debug for HitRecord<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "HitRecord {{ unit: {}, hit: {:?}, normal: {:?}, outside: {} }}",
            self.unit, self.point, self.normal, self.outside
        ))
    }
}

impl<'m> HitRecord<'m> {
    pub fn new<G: Hittable>(r: &Ray, object: &'m G, unit: f64) -> Self {
        let point = r.position_after(unit);
        let mut normal = object.normal(&point);
        let outside = r.direction.dot(&normal) < 0.0;
        if !outside {
            normal.reverse();
        }
        let material = object.material();
        let (u, v) = object.uv(&point);
        Self {
            point,
            normal,
            material,
            unit,
            u,
            v,
            outside,
        }
    }
}

#[allow(unused_variables)]
pub trait Hittable: Send + Sync {
    fn normal(&self, point: &Point3) -> Vec3 {
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

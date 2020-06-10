use {
    crate::{geometry::Geometry, material::Material, prelude::*},
    std::{
        fmt::{Debug, Formatter},
        rc::Rc,
    },
};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f64,
    pub outside: bool,
}

impl Debug for HitRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "HitRecord {{ t: {}, hit: {:?}, normal: {:?}, outside: {} }}",
            self.t, self.point, self.normal, self.outside
        ))
    }
}

impl HitRecord {
    pub fn new<G: Geometry + ?Sized>(r: &Ray, object: &G, t: f64) -> Self {
        let point = r.at(t);
        let mut normal = object.normal(&point);
        let outside = r.direction.dot(&normal) < 0.0;
        if !outside {
            normal.reverse();
        }
        let material = object.material();
        Self {
            point,
            normal,
            material,
            t,
            outside,
        }
    }
}

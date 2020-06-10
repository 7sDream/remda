use {
    crate::{geometry::Geometry, material::Material, prelude::*},
    std::fmt::{Debug, Formatter},
};

pub struct HitRecord<'m> {
    pub point: Point3,
    pub normal: Vec3,
    pub material: &'m dyn Material,
    pub t: f64,
    pub outside: bool,
}

impl Debug for HitRecord<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "HitRecord {{ t: {}, hit: {:?}, normal: {:?}, outside: {} }}",
            self.t, self.point, self.normal, self.outside
        ))
    }
}

impl<'m> HitRecord<'m> {
    pub fn new<G: Geometry>(r: &Ray, object: &'m G, t: f64) -> Self {
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

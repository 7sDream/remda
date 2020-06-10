use {
    super::{Geometry, HitRecord},
    crate::{material::Material, prelude::*},
    std::{
        fmt::{Debug, Formatter},
        ops::Range,
    },
};

pub struct Sphere<M: Material> {
    center: Point3,
    radius: f64,
    material: M,
    radius_squared: f64,
}

impl<M: Material> Debug for Sphere<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Sphere {{ center: {:?}, radius: {} }}",
            self.center, self.radius
        ))
    }
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3, radius: f64, material: M) -> Self {
        Self {
            center,
            radius,
            material,
            radius_squared: radius * radius,
        }
    }
}

impl<M: Material> Geometry for Sphere<M> {
    fn normal(&self, p: &Point3) -> crate::prelude::Vec3 {
        (p - &self.center) / self.radius
    }

    fn material(&self) -> &dyn Material {
        &self.material
    }

    // Ray(t) = O + tD
    // Sphere surface = (X - C)^2 = r^2
    // (O + tD - C)^2 = r^2
    // let O - C = L
    // (tD + L)^2 = r^2
    // D^2 t^2 + 2DLt + L^2- r^2 = 0
    // a = D^2, b = 2(DL), c = L^2 - r^2
    // Delta = b^2 - 4ac = 4(DL)^2 - 4 D^2 (L^2 - r2)
    // So, check (DL)^2 - D^2(L^2 - r^2)
    // root is
    fn hit(&self, ray: &Ray, limit: Range<f64>) -> Option<HitRecord<'_>> {
        let l = &ray.origin - &self.center;
        let half_b = ray.direction.dot(&l);
        let a = ray.direction.length_squared();
        let c = l.length_squared() - self.radius_squared;
        let delta = half_b * half_b - a * c;

        if delta < 0.0 {
            return None;
        }

        let sqrt = delta.sqrt();

        let t1 = (-half_b - sqrt) / a;
        if limit.contains(&t1) {
            return Some(HitRecord::new(ray, self, t1));
        }

        let t2 = (-half_b + sqrt) / a;
        if limit.contains(&t2) {
            return Some(HitRecord::new(ray, self, t2));
        }

        None
    }
}

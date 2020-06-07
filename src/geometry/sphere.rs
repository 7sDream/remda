use {
    super::{Geometry, HitRecord},
    crate::{material::Material, prelude::*},
    std::{ops::Range, rc::Rc},
};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
    radius_squared: f64,
}

impl Sphere {
    pub fn new<M: Material + 'static>(center: Point3, radius: f64, material: M) -> Self {
        let material = Rc::new(material);
        Self { center, radius, material, radius_squared: radius * radius }
    }
}

impl Geometry for Sphere {
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
    fn hit(&self, r: &Ray, limit: Range<f64>) -> Option<HitRecord> {
        let l = &r.origin - &self.center;
        let half_b = r.direction.dot(&l);
        let a = r.direction.length_squared();
        let c = l.length_squared() - self.radius_squared;
        let delta = half_b * half_b - a * c;

        if delta < 0.0 {
            return None;
        }

        let sqrt = delta.sqrt();

        let t = (-half_b - sqrt) / a;
        if limit.contains(&t) {
            return Some(HitRecord::new(r, self, t));
        }

        let t = (-half_b + sqrt) / a;
        if limit.contains(&t) {
            return Some(HitRecord::new(r, self, t));
        }

        None
    }

    fn normal(&self, p: &Point3) -> crate::prelude::Vec3 {
        (p - &self.center).unit()
    }

    fn material(&self) -> Rc<dyn Material> {
        self.material.clone()
    }
}

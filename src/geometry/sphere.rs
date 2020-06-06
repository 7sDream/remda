use {super::Geometry, crate::types::Point3};

pub struct Sphere {
    center: Point3,
    radius: f64,
    radius_squared: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius, radius_squared: radius * radius }
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
    fn hit_time(&self, r: &crate::types::Ray) -> Option<(f64, f64)> {
        let l = &r.origin - &self.center;
        let half_b = r.direction.dot(&l);
        let a = r.direction.length_squared();
        let c = l.length_squared() - self.radius_squared;
        let delta = half_b * half_b - a * c;

        if delta >= 0.0 {
            let sqrt = delta.sqrt();
            Some(((-half_b - sqrt) / a, (-half_b + sqrt) / a))
        } else {
            None
        }
    }

    fn normal(&self, p: &Point3) -> crate::types::Vec3 {
        (p - &self.center).unit()
    }
}

use {super::Geometry, crate::types::Point3, log::info};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Geometry for Sphere {
    // Ray(t) = O + tD
    // Sphere surface = (X - C)^2 = r^2
    // (O + tD - C)^2 = r^2
    // let O - C = L
    // (tD + L)^2 = r^2
    // D^2 t^2 + 2DLt + L^2- r^2 = 0
    // a = D^2, b = 2DL, c = L^2 - r^2
    // Delta = b^2 - 4ac = 4(DL)^2 - 4 D^2 (L^2 - r2)
    // So, check (DL)^2 - D^2(L^2 - r^2)
    fn check_ray_hits(&self, r: &crate::types::Ray) -> bool {
        let l = &r.origin - &self.center;
        let dl2 = r.direction.dot(&l).powi(2);
        let d2 = r.direction.length_squared();
        let l2 = l.length_squared();
        let r2 = self.radius * self.radius;
        let delta = dl2 - d2 * (l2 - r2);
        if r.direction.x.abs() < 0.01 && r.direction.y.abs() < 0.01 {
            info!("ray: {}, delta: {}", r.direction, delta);
            info!("l: {}", l);
            info!("dl^2: {}", dl2);
            info!("l2: {}", l2);
            info!("r2: {}", r2);
        }
        delta > 0.0
    }
}

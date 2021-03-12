use {
    crate::{
        hittable::{Hittable, HitRecord},
        material::Material,
        prelude::*,
    },
    std::{
        fmt::{Debug, Formatter},
        ops::Range,
    },
};

pub struct Sphere<M> {
    center: Point3,
    radius: f64,
    speed: Vec3,
    material: M,
    radius_squared: f64,
}

impl<M> Debug for Sphere<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Sphere {{ center: {:?}, radius: {}, speed: {:?} }}",
            self.center, self.radius, self.speed,
        ))
    }
}

impl<M> Sphere<M> {
    pub fn new(center: Point3, radius: f64, material: M) -> Self {
        Self {
            center,
            radius,
            material,
            speed: Vec3::default(),
            radius_squared: radius * radius,
        }
    }

    pub const fn with_speed(mut self, speed: Vec3) -> Self {
        self.speed = speed;
        self
    }

    pub fn center_at(&self, t: f64) -> Point3 {
        &self.center + &self.speed * t
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn normal(&self, p: &Point3) -> crate::prelude::Vec3 {
        (p - &self.center) / self.radius
    }

    fn material(&self) -> &dyn Material {
        &self.material
    }

    fn uv(&self, point: &Point3) -> (f64, f64) {
        let point = (point - &self.center).unit();
        let phi = (-point.z).atan2(point.x); // [-pi, pi]
        let theta = point.y.asin(); // [-pi / 2 , pi / 2]
        let u = phi / 2.0 / PI + 0.5;
        let v = theta / PI + 0.5;
        (u, v)
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
    fn hit(&self, ray: &Ray, unit_limit: Range<f64>) -> Option<HitRecord<'_>> {
        let current_center = self.center_at(ray.departure_time);
        let l = &ray.origin - current_center;
        let half_b = ray.direction.dot(&l);
        let a = ray.direction.length_squared();
        let c = l.length_squared() - self.radius_squared;
        #[allow(clippy::suspicious_operation_groupings)]
        let delta = half_b * half_b - a * c;

        if delta < 0.0 {
            return None;
        }

        let sqrt = delta.sqrt();

        let t1 = (-half_b - sqrt) / a;
        if unit_limit.contains(&t1) {
            return Some(HitRecord::new(ray, self, t1));
        }

        let t2 = (-half_b + sqrt) / a;
        if unit_limit.contains(&t2) {
            return Some(HitRecord::new(ray, self, t2));
        }

        None
    }

    fn bbox(&self, time_limit: Range<f64>) -> Option<AABB> {
        Some(
            if self.speed.x == 0.0 && self.speed.y == 0.0 && self.speed.z == 0.0 {
                AABB::new(
                    &self.center - Vec3::new(self.radius, self.radius, self.radius),
                    &self.center + Vec3::new(self.radius, self.radius, self.radius),
                )
            } else {
                let start = AABB::new(
                    self.center_at(time_limit.start)
                        - Vec3::new(self.radius, self.radius, self.radius),
                    self.center_at(time_limit.start)
                        + Vec3::new(self.radius, self.radius, self.radius),
                );

                let end = AABB::new(
                    self.center_at(time_limit.end)
                        - Vec3::new(self.radius, self.radius, self.radius),
                    self.center_at(time_limit.end)
                        + Vec3::new(self.radius, self.radius, self.radius),
                );

                start | end
            },
        )
    }
}

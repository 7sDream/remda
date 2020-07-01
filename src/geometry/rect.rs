use {
    super::{Geometry, HitRecord, AABB},
    crate::{material::Material, prelude::*},
    std::ops::Range,
};

#[derive(Debug, Clone)]
pub struct AARectGeometry {
    k: f64,
    a0: f64,
    a1: f64,
    b0: f64,
    b1: f64,
    a_len: f64,
    b_len: f64,
}

impl AARectGeometry {
    #[must_use]
    pub fn new(k: f64, (a0, a1): (f64, f64), (b0, b1): (f64, f64)) -> Self {
        assert!(a0 < a1);
        assert!(b0 < b1);
        Self {
            k,
            a0,
            a1,
            b0,
            b1,
            a_len: a1 - a0,
            b_len: b1 - b0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AARect<M> {
    // 0: a axis, 1: b axis, 2: fixed axis
    axis: (usize, usize, usize),
    geo: AARectGeometry,
    material: M,
}

impl<M> AARect<M> {
    pub const fn new_xy(geo: AARectGeometry, material: M) -> Self {
        Self {
            geo,
            material,
            axis: (0, 1, 2),
        }
    }
}

impl<M: Material> Geometry for AARect<M> {
    fn normal(&self, _p: &Point3) -> Vec3 {
        let mut n = Vec3::default();
        n[self.axis.2] = 1.0;
        n
    }

    fn material(&self) -> &dyn Material {
        &self.material
    }

    fn uv(&self, point: &Point3) -> (f64, f64) {
        (
            (point[self.axis.0] - self.geo.a0) / self.geo.a_len,
            (point[self.axis.1] - self.geo.b0) / self.geo.b_len,
        )
    }

    fn hit(&self, ray: &Ray, unit_limit: Range<f64>) -> Option<HitRecord<'_>> {
        let unit = (self.geo.k - ray.origin[self.axis.2]) / ray.direction[self.axis.2];
        if !unit_limit.contains(&unit) {
            return None;
        }

        let a = unit.mul_add(ray.direction[self.axis.0], ray.origin[self.axis.0]);

        if a < self.geo.a0 || a > self.geo.a1 {
            return None;
        }

        let b = unit.mul_add(ray.direction[self.axis.1], ray.origin[self.axis.1]);

        if b < self.geo.b0 || b > self.geo.b1 {
            return None;
        }

        Some(HitRecord::new(ray, self, unit))
    }

    fn bbox(&self, _time_limit: Range<f64>) -> Option<AABB> {
        let mut p0 = Point3::default();
        p0[self.axis.0] = self.geo.a0;
        p0[self.axis.1] = self.geo.b0;
        p0[self.axis.2] = self.geo.k - 0.0001;

        let mut p1 = Point3::default();
        p1[self.axis.0] = self.geo.a1;
        p1[self.axis.1] = self.geo.b1;
        p1[self.axis.2] = self.geo.k + 0.0001;

        Some(AABB::new(p0, p1))
    }
}

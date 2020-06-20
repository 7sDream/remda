use {
    crate::prelude::*,
    std::ops::{BitOr, BitOrAssign, Range},
};

/// Axis aligned bounding box
#[derive(Debug, Clone)]
pub struct AABB {
    min: Point3,
    max: Point3,
}

impl AABB {
    #[must_use]
    pub const fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    #[must_use]
    pub const fn min(&self) -> &Point3 {
        &self.min
    }

    #[must_use]
    pub const fn max(&self) -> &Point3 {
        &self.max
    }

    #[must_use]
    pub fn hit(&self, ray: &Ray, limit: Range<f64>) -> bool {
        let mut t_min = limit.start;
        let mut t_max = limit.end;
        for i in 0..3 {
            let inv = 1.0 / ray.direction[i];
            let mut t0 = (self.min[i] - ray.origin[i]) * inv;
            let mut t1 = (self.max[i] - ray.origin[i]) * inv;
            if inv < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}

impl BitOr<Self> for &AABB {
    type Output = AABB;

    fn bitor(self, rhs: Self) -> Self::Output {
        let min = Point3::new(
            self.min.x.min(rhs.min.x),
            self.min.y.min(rhs.min.y),
            self.min.z.min(rhs.min.z),
        );

        let max = Point3::new(
            self.max.x.max(rhs.max.x),
            self.max.y.max(rhs.max.y),
            self.max.z.max(rhs.max.z),
        );

        AABB::new(min, max)
    }
}

impl BitOr<Self> for AABB {
    type Output = AABB;

    fn bitor(self, rhs: Self) -> Self::Output {
        &self | &rhs
    }
}

impl BitOr<&Self> for AABB {
    type Output = AABB;

    fn bitor(self, rhs: &Self) -> Self::Output {
        &self | rhs
    }
}

impl BitOr<AABB> for &AABB {
    type Output = AABB;

    fn bitor(self, rhs: AABB) -> Self::Output {
        self | &rhs
    }
}

impl BitOrAssign<&Self> for AABB {
    fn bitor_assign(&mut self, rhs: &AABB) {
        self.min = Point3::new(
            self.min.x.min(rhs.min.x),
            self.min.y.min(rhs.min.y),
            self.min.z.min(rhs.min.z),
        );

        self.max = Point3::new(
            self.max.x.max(rhs.max.x),
            self.max.y.max(rhs.max.y),
            self.max.z.max(rhs.max.z),
        );
    }
}

impl BitOrAssign<Self> for AABB {
    fn bitor_assign(&mut self, rhs: AABB) {
        *self |= &rhs
    }
}

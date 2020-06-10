use super::vec3::{Point3, Vec3};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    #[must_use]
    pub const fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    #[must_use]
    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + &self.direction * t
    }
}

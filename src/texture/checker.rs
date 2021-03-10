use crate::{
    prelude::{Color, Point3},
    texture::Texture,
};

#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct Checker<T1, T2> {
    odd: T1,
    even: T2,
}

impl<T1, T2> Checker<T1, T2> {
    #[must_use]
    pub const fn new(odd: T1, even: T2) -> Self {
        Self { odd, even }
    }
}

impl<T1: Texture, T2: Texture> Texture for Checker<T1, T2> {
    fn color(&self, u: f64, v: f64, point: &Point3) -> Color {
        let value = (10.0 * point.x).sin() * (10.0 * point.y).sin() * (10.0 * point.z).sin();
        if value < 0.0 {
            self.odd.color(u, v, point)
        } else {
            self.even.color(u, v, point)
        }
    }
}

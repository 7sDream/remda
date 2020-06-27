use crate::prelude::*;

pub(crate) mod checker;
pub(crate) mod noise;

pub use {checker::Checker, noise::Perlin};

pub trait Texture: Send + Sync {
    fn color(&self, u: f64, v: f64, point: &Point3) -> Color;
}

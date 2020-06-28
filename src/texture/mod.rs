use crate::prelude::*;

pub(crate) mod checker;
pub(crate) mod image;
pub(crate) mod noise;

pub use {
    self::image::Image,
    checker::Checker,
    noise::{Perlin, SmoothType},
};

pub trait Texture: Send + Sync {
    fn color(&self, u: f64, v: f64, point: &Point3) -> Color;
}

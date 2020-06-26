use crate::prelude::*;

pub(crate) mod checker;

pub use checker::Checker;

pub trait Texture: Send + Sync {
    fn color(&self, u: f64, v: f64, point: &Point3) -> Color;
}

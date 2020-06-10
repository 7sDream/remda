use rand::RngCore;
use {
    rand::{rngs::StdRng, Rng, SeedableRng},
    std::ops::Range,
};

#[must_use]
fn normal<R: Rng>(mut rng: R) -> f64 {
    rng.gen_range(0.0, 1.0)
}

#[must_use]
fn range<R: Rng>(mut rng: R, r: Range<f64>) -> f64 {
    rng.gen_range(r.start, r.end)
}

#[derive(Debug)]
pub struct Random();

impl Random {
    #[must_use]
    pub fn normal() -> f64 {
        normal(rand::thread_rng())
    }

    #[must_use]
    pub fn range(r: Range<f64>) -> f64 {
        range(rand::thread_rng(), r)
    }
}

#[derive(Debug)]
pub struct SeedRandom(StdRng);

impl SeedRandom {
    #[must_use]
    pub fn new(seed: u64) -> Self {
        Self(StdRng::seed_from_u64(seed))
    }

    #[must_use]
    pub fn random() -> Self {
        Self::new(rand::thread_rng().next_u64())
    }

    pub fn normal(&mut self) -> f64 {
        normal(&mut self.0)
    }

    pub fn range(&mut self, r: Range<f64>) -> f64 {
        range(&mut self.0, r)
    }
}

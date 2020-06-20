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

fn choose<T, R: Rng, S: AsRef<[T]>>(mut rng: R, values: &S) -> &T {
    let slice = values.as_ref();
    assert!(slice.len() > 0);
    let index = rng.gen_range(0, slice.len());
    &slice[index]
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

    pub fn choose<T, S: AsRef<[T]>>(values: &S) -> &T {
        choose(rand::thread_rng(), values)
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

    pub fn choose<'i, 's, T, S: AsRef<[T]>>(&'i mut self, values: &'s S) -> &'s T {
        choose(&mut self.0, values)
    }
}

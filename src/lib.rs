#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(missing_debug_implementations, rust_2018_idioms)]
#![deny(warnings)]
#![allow(clippy::module_name_repetitions, clippy::cast_possible_truncation)]

pub mod camera;
pub mod geometry;
pub mod image;
pub mod material;
pub mod prelude;
pub mod texture;

mod internal;

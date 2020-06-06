#![deny(warnings)]
#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(missing_debug_implementations, rust_2018_idioms)]
#![allow(clippy::module_name_repetitions)]
#![allow(dead_code)]

use env_logger;

mod camera;
mod geometry;
mod image;
mod material;
mod prelude;

use {
    camera::Camera,
    geometry::{Sphere, World},
    prelude::*,
};

fn main() {
    env_logger::init();

    let origin = Point3::default();
    let camera = Camera::new(origin.clone());
    let mut world = World::new();
    world
        .add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5))
        .add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    camera
        .painter(384)
        .set_samples(100)
        .draw("first.ppm", |u, v| {
            let r = camera.ray(u, v);
            material::diffuse(&r, &world, 50)
        })
        .unwrap();
}

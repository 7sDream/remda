#![deny(warnings)]
#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(missing_debug_implementations, rust_2018_idioms)]
#![allow(clippy::module_name_repetitions)]
#![allow(dead_code)]

use env_logger;

mod camera;
mod geometry;
mod image;
mod prelude;

use {
    camera::Camera,
    geometry::{Geometry, Sphere, World},
    prelude::*,
};

fn normal_color(normal: &Vec3) -> Vec3 {
    (normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5
}

fn bg_color(r: &Ray) -> Vec3 {
    let unit = r.direction.unit();
    let t = 0.5 * (unit.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

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
            if let Some(record) = world.hit(&r, 0.0..f64::INFINITY) {
                normal_color(&record.normal)
            } else {
                bg_color(&r)
            }
        })
        .unwrap();
}

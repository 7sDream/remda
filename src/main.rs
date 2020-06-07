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
    geometry::{Geometry, Sphere, World},
    material::Lambertian,
    prelude::*,
};

fn normal_color(normal: &Vec3) -> Vec3 {
    (normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5
}

fn background(r: &Ray) -> Color {
    let unit = r.direction.unit();
    let t = 0.5 * (unit.y + 1.0);
    Color::newf(1.0, 1.0, 1.0).gradient(&Color::newf(0.5, 0.7, 1.0), t)
}

fn ray_color(r: &Ray, world: &World, depth: usize) -> Color {
    if depth == 0 {
        return Color::default();
    }
    if let Some(hit) = world.hit(r, 0.001..INFINITY) {
        let material = hit.material.clone();
        if let Some(scattered) = material.scatter(r, hit) {
            return scattered.color * ray_color(&scattered.ray, world, depth - 1);
        }
        return Color::default();
    }

    background(r)
}

fn main() {
    env_logger::init();

    let origin = Point3::default();
    let camera = Camera::new(origin.clone());
    let mut world = World::new();
    world
        .add(Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Lambertian::new(Color::newf(0.5, 0.5, 0.5)),
        ))
        .add(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Lambertian::new(Color::newf(0.5, 0.5, 0.5)),
        ));

    camera
        .painter(384)
        .set_samples(256)
        .draw("first.ppm", |u, v| {
            let r = camera.ray(u, v);
            ray_color(&r, &world, 50).into()
        })
        .unwrap();
}

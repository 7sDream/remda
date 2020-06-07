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
    material::{Dielectric, Glass, Lambertian, Metal},
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

fn make_world() -> World {
    let ground = Lambertian::new(Color::newf(0.5, 0.5, 0.5));
    let mut world = World::new();
    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground));

    let small_ball_radius = 0.2;
    let elusion = Point3::new(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let mat = Random::normal();
            let center = Point3::new(
                a as f64 + 0.9 * Random::normal(),
                0.2,
                b as f64 + 0.9 * Random::normal(),
            );

            if (&center - &elusion).length() > 0.9 {
                if mat < 0.8 {
                    let color = Color::newf(Random::normal(), Random::normal(), Random::normal());
                    let material = Lambertian::new(color);
                    world.add(Sphere::new(center, small_ball_radius, material));
                } else if mat < 0.95 {
                    let color = Color::newf(
                        Random::range(0.5..1.0),
                        Random::range(0.5..1.0),
                        Random::range(0.5..1.0),
                    );
                    let fuzz = Random::range(0.0..0.5);
                    let material = Metal::new(color).fuzz(fuzz);
                    world.add(Sphere::new(center, small_ball_radius, material));
                } else {
                    world.add(Sphere::new(
                        center,
                        small_ball_radius,
                        Dielectric::new(Color::newf(1.0, 1.0, 1.0), 1.5, Glass {}),
                    ));
                }
            }
        }
    }

    world.add(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(Color::newf(1.0, 1.0, 1.0), 1.5, Glass {}),
    ));

    world.add(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(Color::newf(0.4, 0.2, 0.1)),
    ));

    world.add(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Color::newf(0.7, 0.6, 0.5)),
    ));

    world
}

#[allow(unused_variables)]
fn main() {
    env_logger::init();
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_to = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let fov = 20.0;
    let aspect_ratio = 16.0 / 9.0;
    let aperture = 0.1;
    let focus = 10.0;
    let camera = Camera::new(look_from, look_to, vup, fov, aspect_ratio, aperture, focus);
    let world = make_world();
    camera
        .painter(1080)
        .set_samples(512)
        .draw("first.ppm", |u, v| {
            let r = camera.ray(u, v);
            ray_color(&r, &world, 50).into()
        })
        .unwrap();
}

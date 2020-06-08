#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(missing_debug_implementations, rust_2018_idioms)]
#![allow(clippy::module_name_repetitions, dead_code)]

mod camera;
mod geometry;
mod image;
mod material;
mod prelude;

use {
    camera::CameraBuilder,
    geometry::{Sphere, World},
    material::{Dielectric, Glass, Lambertian, Metal},
    prelude::*,
};

fn add_small_balls(world: &mut World) {
    let small_ball_radius = 0.2;
    let avoid = Point3::new(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let mat = Random::normal();
            let center = Point3::new(
                0.9_f64.mul_add(Random::normal(), f64::from(a)),
                0.2,
                0.9_f64.mul_add(Random::normal(), f64::from(b)),
            );

            if (&center - &avoid).length() > 0.9 {
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
}

fn add_big_balls(world: &mut World) {
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
}

fn make_world() -> World {
    let mut world = World::new();

    // Ground
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(Color::newf(0.5, 0.5, 0.5)),
    ));

    add_small_balls(&mut world);
    add_big_balls(&mut world);

    world
}

fn main() {
    env_logger::init();

    let camera = CameraBuilder::default()
        .look_from(Point3::new(13.0, 2.0, 3.0))
        .look_at(Point3::new(0.0, 0.0, 0.0))
        .fov(20.0)
        .aperture(0.1)
        .focus(10.0)
        .build();

    let world = make_world();

    camera
        .take_photo(&world)
        .height(108)
        .samples(256)
        .shot("rendered.ppm")
        .unwrap();
}

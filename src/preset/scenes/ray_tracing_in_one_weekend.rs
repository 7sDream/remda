use {
    crate::{
        camera::{CameraBuilder, Camera},
        geometry::{Sphere, World},
        material::{Dielectric, Glass, Lambertian, Metal},
        prelude::*,
    },
};

fn add_small_balls(world: &mut World, rng: &mut SeedRandom) {
    let small_ball_radius = 0.2;
    let avoid = Point3::new(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let mat = rng.normal();
            let center = Point3::new(
                0.9_f64.mul_add(rng.normal(), f64::from(a)),
                0.2,
                0.9_f64.mul_add(rng.normal(), f64::from(b)),
            );

            if (&center - &avoid).length() > 0.9 {
                if mat < 0.8 {
                    let color = Color::newf(rng.normal(), rng.normal(), rng.normal());
                    let material = Lambertian::new(color);
                    world.add(Sphere::new(center, small_ball_radius, material));
                } else if mat < 0.95 {
                    let color = Color::newf(
                        rng.range(0.5..1.0),
                        rng.range(0.5..1.0),
                        rng.range(0.5..1.0),
                    );
                    let fuzz = rng.range(0.0..0.5);
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

#[must_use]
pub fn world(seed: Option<u64>) -> World {
    let mut world = World::default();

    // Ground
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(Color::newf(0.5, 0.5, 0.5)),
    ));

    let mut rng = if let Some(seed) = seed {
        SeedRandom::new(seed)
    } else {
        SeedRandom::random()
    };

    add_small_balls(&mut world, &mut rng);
    add_big_balls(&mut world);

    world
}

#[must_use]
pub fn camera() -> Camera {
    CameraBuilder::default()
        .look_from(Point3::new(13.0, 2.0, 3.0))
        .look_at(Point3::new(0.0, 0.0, 0.0))
        .fov(20.0)
        .aperture(0.1)
        .focus(10.0)
        .build()
}

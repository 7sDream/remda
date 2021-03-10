use remda::{
    camera::{Camera, CameraBuilder},
    geometry::{collection::GeometryList, Sphere},
    material::{Dielectric, Glass, Lambertian, Metal},
    prelude::*,
    texture::Checker,
};

fn add_small_balls(world: &mut GeometryList, rng: &mut SeedRandom, need_speed: bool) {
    let small_ball_radius = 0.2;
    let mut avoid = Point3::new(0.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                0.9_f64.mul_add(rng.normal(), f64::from(a)),
                0.2,
                0.9_f64.mul_add(rng.normal(), f64::from(b)),
            );

            avoid.x = center.x;

            if !((0.0..0.9).contains(&center.x.abs()) || (3.1..4.9).contains(&center.x.abs()))
                || (&center - &avoid).length() >= 0.9
            {
                let mat = rng.normal();
                if mat < 0.8 {
                    let color = Color::new(rng.normal(), rng.normal(), rng.normal());
                    let material = Lambertian::new(color);
                    let mut sphere = Sphere::new(center, small_ball_radius, material);
                    if need_speed {
                        sphere = sphere.with_speed(Vec3::new(0.0, Random::range(0.0..0.5), 0.0));
                    }
                    world.add(sphere);
                } else if mat < 0.95 {
                    let color = Color::new(
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
                        Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.5).reflect_curve(Glass {}),
                    ));
                }
            }
        }
    }
}

fn add_big_balls(world: &mut GeometryList) {
    world.add(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.5).reflect_curve(Glass {}),
    ));

    world.add(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(Color::new(0.4, 0.2, 0.1)),
    ));

    world.add(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Color::new(0.7, 0.6, 0.5)),
    ));
}

#[must_use]
pub fn world(seed: Option<u64>, need_speed: bool, checker: bool) -> GeometryList {
    let mut list = GeometryList::default();

    if checker {
        list.add(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian::new(Checker::new(
                Color::new(0.2, 0.3, 0.1),
                Color::new(0.9, 0.9, 0.9),
            )),
        ));
    } else {
        list.add(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian::new(Color::new(0.5, 0.5, 0.5)),
        ));
    };

    // Ground

    let mut rng = if let Some(seed) = seed {
        SeedRandom::new(seed)
    } else {
        SeedRandom::random()
    };

    add_small_balls(&mut list, &mut rng, need_speed);
    add_big_balls(&mut list);

    list
}

#[must_use]
pub fn camera(need_shutter_speed: bool) -> Camera {
    let mut builder = CameraBuilder::default()
        .look_from(Point3::new(13.0, 2.0, 3.0))
        .look_at(Point3::new(0.0, 0.0, 0.0))
        .fov(20.0)
        .aperture(0.1)
        .focus(10.0);

    if need_shutter_speed {
        builder = builder.shutter_speed(1.0);
    }

    builder.build()
}

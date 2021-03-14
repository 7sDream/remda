use {
    remda::{
        camera::{Camera, CameraBuilder},
        hittable::{
            collection::{HittableList, BVH},
            medium::ConstantMedium,
            transform::{AARotation, ByYAxis, Translation},
            AARect, AARectMetrics, Carton, Sphere,
        },
        material::{Dielectric, DiffuseLight, Glass, Lambertian, Metal},
        prelude::*,
        texture::{Checker, Image, Perlin, SmoothType},
    },
    std::sync::Arc,
};

fn add_small_balls(world: &mut HittableList, rng: &mut SeedRandom, need_speed: bool) {
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

fn add_big_balls(world: &mut HittableList) {
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
pub fn balls_scene(seed: Option<u64>, need_speed: bool, checker: bool) -> HittableList {
    let mut list = HittableList::default();

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
pub fn balls_scene_camera(need_shutter_speed: bool) -> Camera {
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

#[must_use]
pub fn cornell_box_scene(
    carton: bool, carton_rotation: bool, smoke: bool,
) -> (Camera, HittableList) {
    let red = Lambertian::new(Color::new(0.65, 0.05, 0.05));
    let green = Lambertian::new(Color::new(0.12, 0.45, 0.15));
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let light =
        DiffuseLight::new(Color::new(1.0, 1.0, 1.0)).multiplier(if smoke { 7.0 } else { 15.0 });

    let mut objects = HittableList::default();

    objects
        .add(AARect::new_yz(
            AARectMetrics::new(555.0, (0.0, 555.0), (0.0, 555.0)),
            green,
        ))
        .add(AARect::new_yz(
            AARectMetrics::new(0.0, (0.0, 555.0), (0.0, 555.0)),
            red,
        ))
        .add(AARect::new_xz(
            AARectMetrics::new(0.0, (0.0, 555.0), (0.0, 555.0)),
            white.clone(),
        ))
        .add(AARect::new_xz(
            AARectMetrics::new(555.0, (0.0, 555.0), (0.0, 555.0)),
            white.clone(),
        ))
        .add(AARect::new_xy(
            AARectMetrics::new(555.0, (0.0, 555.0), (0.0, 555.0)),
            white.clone(),
        ));

    if smoke {
        objects.add(AARect::new_xz(
            AARectMetrics::new(554.0, (113.0, 443.0), (127.0, 432.0)),
            light,
        ));
    } else {
        objects.add(AARect::new_xz(
            AARectMetrics::new(554.0, (213.0, 343.0), (227.0, 332.0)),
            light,
        ));
    }

    if carton {
        if carton_rotation {
            let box1 = Translation::new(
                AARotation::<ByYAxis, _>::new(
                    Carton::new(
                        Point3::new(0.0, 0.0, 0.0),
                        Point3::new(165.0, 165.0, 165.0),
                        white.clone(),
                    ),
                    -18.0,
                ),
                Vec3::new(130.0, 0.0, 65.0),
            );
            let box2 = Translation::new(
                AARotation::<ByYAxis, _>::new(
                    Carton::new(
                        Point3::new(0.0, 0.0, 0.0),
                        Point3::new(165.0, 330.0, 165.0),
                        white,
                    ),
                    15.0,
                ),
                Vec3::new(265.0, 0.0, 295.0),
            );
            if smoke {
                let box1 = ConstantMedium::new(box1, Color::new(1.0, 1.0, 1.0), 0.01);
                let box2 = ConstantMedium::new(box2, Color::new(0.0, 0.0, 0.0), 0.01);
                objects.add(box1).add(box2);
            } else {
                objects.add(box1).add(box2);
            }
        } else {
            let box1 = Carton::new(
                Point3::new(130.0, 0.0, 65.0),
                Point3::new(295.0, 165.0, 230.0),
                white.clone(),
            );
            let box2 = Carton::new(
                Point3::new(265.0, 0.0, 295.0),
                Point3::new(430.0, 330.0, 460.0),
                white,
            );
            if smoke {
                let box1 = ConstantMedium::new(box1, Color::new(1.0, 1.0, 1.0), 0.01);
                let box2 = ConstantMedium::new(box2, Color::new(0.0, 0.0, 0.0), 0.01);
                objects.add(box1).add(box2);
            } else {
                objects.add(box1).add(box2);
            }
        }
    }

    let camera = CameraBuilder::default()
        .aspect_ratio(1.0)
        .fov(40.0)
        .look_from(Point3::new(278.0, 278.0, -800.0))
        .look_at(Point3::new(278.0, 278.0, 0.0))
        .build();

    (camera, objects)
}

pub fn all_feature_scene(seed: Option<u64>) -> (Camera, HittableList) {
    let time_limit = 0.0..1.0;
    let boxes_per_side: usize = 20;
    let mut rng = seed.map(SeedRandom::new).unwrap_or_default();

    let mut boxes1 = HittableList::default();
    let ground = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.range(1.0..100.0);
            let z1 = z0 + w;
            boxes1.add(Carton::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                Arc::clone(&ground),
            ));
        }
    }

    let mut objects = HittableList::default();
    objects.add(BVH::new(boxes1, time_limit.clone()));

    let light = DiffuseLight::new(Color::new(1.0, 1.0, 1.0)).multiplier(7.0);
    objects.add(AARect::new_xz(
        AARectMetrics::new(554.0, (123.0, 423.0), (147.0, 412.0)),
        light,
    ));

    let moving_sphere = Sphere::new(
        Point3::new(400.0, 400.0, 200.0),
        50.0,
        Lambertian::new(Color::new(0.7, 0.3, 0.1)),
    )
    .with_speed(Vec3::new(30.0, 0.0, 0.0));
    objects.add(moving_sphere);

    let glass_sphere = Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.5).reflect_curve(Glass {}),
    );
    objects.add(glass_sphere);

    let metal_sphere = Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Metal::new(Color::new(0.8, 0.8, 0.9)).fuzz(1.0),
    );
    objects.add(metal_sphere);

    let boundary = Sphere::new(
        Point3::new(360.0, 170.0, 145.0),
        70.0,
        Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.5).reflect_curve(Glass {}),
    );
    objects.add(boundary);
    objects.add(ConstantMedium::new(
        Sphere::new(
            Point3::new(360.0, 170.0, 145.0),
            70.0,
            Lambertian::new(Color::new(1.0, 1.0, 1.0)),
        ),
        Color::new(0.2, 0.4, 0.9),
        0.2,
    ));

    objects.add(ConstantMedium::new(
        Sphere::new(
            Point3::new(0.0, 0.0, 0.0),
            5000.0,
            Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.5).reflect_curve(Glass {}),
        ),
        Color::new(1.0, 1.0, 1.0),
        0.0001,
    ));

    objects.add(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        Lambertian::new(Image::new("examples/earth-map.png").unwrap()),
    ));

    objects.add(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Lambertian::new(
            Perlin::new(256, true)
                .scale(0.1)
                .smooth(SmoothType::HermitianCubic),
        ),
    ));

    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let mut boxes2 = HittableList::default();
    for _ in 0..1000_usize {
        boxes2.add(Sphere::new(
            Point3::new(
                rng.range(0.0..165.0),
                rng.range(0.0..165.0),
                rng.range(0.0..165.0),
            ),
            10.0,
            Arc::clone(&white),
        ));
    }

    objects.add(Translation::new(
        AARotation::<ByYAxis, _>::new(BVH::new(boxes2, time_limit), 15.0),
        Vec3::new(-100.0, 270.0, 395.0),
    ));

    let camera = CameraBuilder::default()
        .look_from(Point3::new(478.0, 278.0, -600.0))
        .look_at(Point3::new(278.0, 278.0, 0.0))
        .aspect_ratio(1.0)
        .fov(40.0)
        .shutter_speed(1.0)
        .build();

    (camera, objects)
}

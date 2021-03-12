use remda::{
    camera::CameraBuilder,
    hittable::{collection::HittableList, Sphere},
    material::{Dielectric, Glass, Lambertian, Metal},
    prelude::*,
};

fn main() {
    env_logger::init();

    let mut world = HittableList::default();

    world
        .add(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Lambertian::new(Color::new(0.8, 0.8, 0.0)),
        ))
        .add(Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Lambertian::new(Color::new(0.1, 0.2, 0.5)),
        ))
        .add(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            Metal::new(Color::new(0.8, 0.6, 0.2)),
        ))
        .add(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.5).reflect_curve(Glass {}),
        ))
        .add(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            -0.45,
            Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.5).reflect_curve(Glass {}),
        ));

    let camera = CameraBuilder::default()
        .look_from(Point3::new(-2.0, 2.0, 1.0))
        .fov(20.0)
        .aspect_ratio(2.0)
        .build();

    camera
        .take_photo(world)
        .height(100)
        .samples(100)
        .shot(Some("rtow_11_2_2.ppm"))
        .unwrap();
}

use remda::{
    camera::CameraBuilder,
    geometry::{Sphere, World},
    material::{Dielectric, Glass, Lambertian, Metal},
    prelude::*,
};

fn main() {
    env_logger::init();

    let mut world = World::default();
    world.set_bg(|ray| {
        let unit = ray.direction.unit();
        let t = 0.5 * (unit.y + 1.0);
        Color::newf(1.0, 1.0, 1.0).gradient(&Color::newf(0.5, 0.7, 1.0), t)
    });

    world
        .add(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Lambertian::new(Color::newf(0.8, 0.8, 0.0)),
        ))
        .add(Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Lambertian::new(Color::newf(0.1, 0.2, 0.5)),
        ))
        .add(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            Metal::new(Color::newf(0.8, 0.6, 0.2)).fuzz(0.3),
        ))
        .add(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            Dielectric::new(Color::newf(1.0, 1.0, 1.0), 1.5).reflect_curve(Glass {}),
        ))
        .add(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            -0.45,
            Dielectric::new(Color::newf(1.0, 1.0, 1.0), 1.5).reflect_curve(Glass {}),
        ));

    let camera = CameraBuilder::default().aspect_ratio(2.0).build();

    camera
        .take_photo(&world)
        .height(100)
        .samples(100)
        .shot(Some("rtow_10_5.ppm"))
        .unwrap();
}

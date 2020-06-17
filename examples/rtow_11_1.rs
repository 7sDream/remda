use remda::{
    camera::CameraBuilder,
    geometry::{Sphere, World},
    material::Lambertian,
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

    let r = (PI / 4.0).cos();

    world
        .add(Sphere::new(
            Point3::new(-r, 0.0, -1.0),
            r,
            Lambertian::new(Color::newf(0.0, 0.0, 1.0)),
        ))
        .add(Sphere::new(
            Point3::new(r, 0.0, -1.0),
            r,
            Lambertian::new(Color::newf(1.0, 0.0, 0.0)),
        ));

    let camera = CameraBuilder::default().aspect_ratio(2.0).build();

    camera
        .take_photo(&world)
        .height(100)
        .samples(100)
        .shot(Some("rtow_11_1.ppm"))
        .unwrap();
}

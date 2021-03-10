use remda::{
    camera::CameraBuilder,
    geometry::{collection::GeometryList, Sphere},
    material::{Lambertian, Metal},
    prelude::*,
};

fn main() {
    env_logger::init();

    let mut world = GeometryList::default();

    world
        .add(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Lambertian::new(Color::new(0.8, 0.8, 0.0)),
        ))
        .add(Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Lambertian::new(Color::new(0.7, 0.3, 0.3)),
        ))
        .add(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            Metal::new(Color::new(0.8, 0.6, 0.2)),
        ))
        .add(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            Metal::new(Color::new(0.8, 0.8, 0.8)),
        ));

    let camera = CameraBuilder::default().aspect_ratio(2.0).build();

    camera
        .take_photo(world)
        .height(100)
        .samples(100)
        .shot(Some("rtow_9_5.ppm"))
        .unwrap();
}

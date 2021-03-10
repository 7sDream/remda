use remda::{
    camera::CameraBuilder,
    geometry::{collection::GeometryList, Sphere},
    material::Lambertian,
    prelude::*,
};

fn main() {
    env_logger::init();

    let mut world = GeometryList::default();

    let r = (PI / 4.0).cos();

    world
        .add(Sphere::new(
            Point3::new(-r, 0.0, -1.0),
            r,
            Lambertian::new(Color::new(0.0, 0.0, 1.0)),
        ))
        .add(Sphere::new(
            Point3::new(r, 0.0, -1.0),
            r,
            Lambertian::new(Color::new(1.0, 0.0, 0.0)),
        ));

    let camera = CameraBuilder::default().aspect_ratio(2.0).build();

    camera
        .take_photo(world)
        .height(100)
        .samples(100)
        .shot(Some("rtow_11_1.ppm"))
        .unwrap();
}

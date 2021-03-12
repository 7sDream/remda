use remda::{
    camera::CameraBuilder,
    hittable::{collection::HittableList, Sphere},
    material::{Lambertian, LambertianMathType},
    prelude::*,
};

fn main() {
    env_logger::init();

    let mut world = HittableList::default();

    world
        .add(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Lambertian::new(Color::new(0.5, 0.5, 0.5)).math_type(LambertianMathType::Hemisphere),
        ))
        .add(Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            Lambertian::new(Color::new(0.5, 0.5, 0.5)).math_type(LambertianMathType::Hemisphere),
        ));

    let camera = CameraBuilder::default().aspect_ratio(2.0).build();

    camera
        .take_photo(world)
        .height(100)
        .samples(100)
        .shot(Some("rtow_8_6.ppm"))
        .unwrap();
}

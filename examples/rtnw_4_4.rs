use remda::{
    camera::CameraBuilder,
    hittable::{collection::HittableList, Sphere},
    material::Lambertian,
    prelude::*,
    texture::Checker,
};

fn main() {
    let mut world = HittableList::default();
    let checker = Checker::new(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));

    world
        .add(Sphere::new(
            Point3::new(0.0, -10.0, 0.0),
            10.0,
            Lambertian::new(checker.clone()),
        ))
        .add(Sphere::new(
            Point3::new(0.0, 10.0, 0.0),
            10.0,
            Lambertian::new(checker),
        ));

    let camera = CameraBuilder::default()
        .aspect_ratio(5.0 / 3.0)
        .fov(20.0)
        .look_from(Point3::new(13.0, 2.0, 3.0))
        .look_at(Point3::new(0.0, 0.0, 0.0))
        .focus(10.0)
        .build();

    camera
        .take_photo(world)
        .height(300)
        .samples(128)
        .shot(Some("rtnw_4_4.ppm"))
        .unwrap();
}

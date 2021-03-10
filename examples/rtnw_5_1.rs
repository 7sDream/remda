use remda::{
    camera::CameraBuilder,
    geometry::{collection::GeometryList, Sphere},
    material::Lambertian,
    prelude::*,
    texture::{Perlin, SmoothType},
};

fn main() {
    let mut world = GeometryList::default();
    let perlin = Perlin::new(256, false).smooth(SmoothType::None);

    world
        .add(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian::new(perlin.clone()),
        ))
        .add(Sphere::new(
            Point3::new(0.0, 2.0, 0.0),
            2.0,
            Lambertian::new(perlin),
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
        .shot(Some("rtnw_5_1.ppm"))
        .unwrap();
}

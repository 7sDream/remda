use remda::{
    camera::CameraBuilder,
    geometry::{GeometryList, Sphere},
    material::Lambertian,
    prelude::*,
    texture::Image,
};

fn main() {
    let mut world = GeometryList::default();
    // Image comes from https://en.wikipedia.org/wiki/Equirectangular_projection
    let earth_texture = Image::new("examples/earthmap.png").unwrap();

    world
        .add(Sphere::new(
            Point3::new(0.0, 0.0, 0.0),
            3.0,
            Lambertian::new(earth_texture),
        ))
        .add(Sphere::new(
            Point3::new(0.0, -1003.0, 0.0),
            1000.0,
            Lambertian::new(Color::newf(0.5, 0.5, 0.5)),
        ));

    let camera = CameraBuilder::default()
        .aspect_ratio(1.0)
        .fov(90.0)
        .look_from(Point3::new(0.0, 4.0, -6.0))
        .look_at(Point3::new(0.0, 0.0, 0.0))
        .build();

    camera
        .take_photo(world)
        .height(800)
        .samples(128)
        .shot(Some("rtnw_6_2.ppm"))
        .unwrap();
}

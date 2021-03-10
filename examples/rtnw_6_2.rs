use remda::{
    camera::CameraBuilder,
    geometry::{collection::GeometryList, Sphere},
    material::Lambertian,
    prelude::*,
    texture::Image,
};

fn main() {
    let mut world = GeometryList::default();
    // Image comes from http://visibleearth.nasa.gov/view.php?id=57752
    let earth_texture = Image::new("examples/earth-map.png").unwrap();

    world
        .add(Sphere::new(
            Point3::new(0.0, 0.0, 0.0),
            3.0,
            Lambertian::new(earth_texture),
        ))
        .add(Sphere::new(
            Point3::new(0.0, -1003.0, 0.0),
            1000.0,
            Lambertian::new(Color::new(0.5, 0.5, 0.5)),
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

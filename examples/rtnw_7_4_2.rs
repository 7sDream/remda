#[allow(dead_code)]
mod common;

use remda::{
    camera::CameraBuilder,
    hittable::{collection::HittableList, AARect, AARectMetrics, Sphere},
    material::{DiffuseLight, Lambertian},
    prelude::*,
    texture::Perlin,
};

fn main() {
    common::init_log("info");

    let mut world = HittableList::default();
    let perlin = Perlin::new(256, true).scale(4.0).marble(7);

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
        ))
        .add(Sphere::new(
            Point3::new(0.0, 7.0, 0.0),
            2.0,
            DiffuseLight::new(Color::new(1.0, 1.0, 1.0)).multiplier(4.0),
        ))
        .add(AARect::new_xy(
            AARectMetrics::new(-2.0, (3.0, 5.0), (1.0, 3.0)),
            DiffuseLight::new(Color::new(1.0, 1.0, 1.0)).multiplier(4.0),
        ));

    let camera = CameraBuilder::default()
        .aspect_ratio(5.0 / 3.0)
        .fov(20.0)
        .look_from(Point3::new(25.0, 5.0, 7.5))
        .look_at(Point3::new(0.0, 2.0, 0.0))
        .focus(10.0)
        .build();

    camera
        .take_photo(world)
        .background(|_| Color::default())
        .height(300)
        .samples(1000)
        .shot(Some("rtnw_7_4_2.ppm"))
        .unwrap();
}

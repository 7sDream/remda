#[allow(dead_code)]
mod common;

use remda::{
    camera::CameraBuilder,
    geometry::{Carton, Geometry, GeometryList},
    material::Lambertian,
    prelude::*,
};

fn main() {
    let camera = CameraBuilder::default()
        .look_from(Point3::new(-1.0, 1.5, 2.0))
        .look_at(Point3::new(0.0, 0.0, 0.5))
        .build();

    let mut world = GeometryList::default();
    let carton_at_origin = Carton::new(
        Point3::new(-0.5, 0.0, -0.5),
        Point3::new(0.5, 1.0, 0.5),
        Lambertian::new(Color::newf(0.2, 0.2, 0.2)),
    );
    let carton_moved = carton_at_origin.clone().translate(Vec3::new(2.0, 0.0, 0.0));
    world.add(carton_at_origin).add(carton_moved);

    camera
        .take_photo(world)
        .height(480)
        .samples(512)
        .shot(Some("rtnw_8_1.ppm"))
        .unwrap();
}

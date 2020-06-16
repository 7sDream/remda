use remda::{camera::CameraBuilder, geometry::World, prelude::*};

fn main() {
    env_logger::init();

    let mut world = World::default();
    world.set_bg(|ray| {
        let unit = ray.direction.unit();
        let t = 0.5 * (unit.y + 1.0);
        Color::newf(1.0, 1.0, 1.0).gradient(&Color::newf(0.5, 0.7, 1.0), t)
    });

    let camera = CameraBuilder::default().build();

    camera
        .take_photo(&world)
        .height(216)
        .gamma(false)
        .samples(1)
        .shot(Some("rtow_4_2.ppm"))
        .unwrap();
}

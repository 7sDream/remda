use remda::{camera::CameraBuilder, geometry::World, prelude::*};

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> bool {
    let oc = &ray.origin - center;
    let a = ray.direction.length_squared();
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn main() {
    env_logger::init();

    let mut world = World::default();
    world.set_bg(|ray| {
        if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray) {
            return Color::newf(1.0, 0.0, 0.0);
        }
        let unit = ray.direction.unit();
        let t = 0.5 * (unit.y + 1.0);
        Color::newf(1.0, 1.0, 1.0).gradient(&Color::newf(0.5, 0.7, 1.0), t)
    });

    let camera = CameraBuilder::default().build();

    camera
        .take_photo(&world)
        .height(432)
        .gamma(false)
        .samples(1)
        .shot(Some("rtow_5_2.ppm"))
        .unwrap();
}

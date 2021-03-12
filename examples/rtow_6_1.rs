use remda::{camera::CameraBuilder, hittable::collection::HittableList, prelude::*};

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> Option<f64> {
    let oc = &ray.origin - center;
    let a = ray.direction.length_squared();
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        None
    } else {
        Some((-b - discriminant.sqrt()) / (2.0 * a))
    }
}

fn background(ray: &Ray) -> Color {
    if let Some(t) = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray) {
        if t > 0.0 {
            let n = (ray.position_after(t) - Vec3::new(0.0, 0.0, -1.0)).unit();
            return (0.5 * (n + Vec3::new(1.0, 1.0, 1.0))).into_color(1, false);
        }
    }
    let unit = ray.direction.unit();
    let t = 0.5 * (unit.y + 1.0);
    Color::new(1.0, 1.0, 1.0).gradient(&Color::new(0.5, 0.7, 1.0), t)
}

fn main() {
    env_logger::init();

    let world = HittableList::default();

    let camera = CameraBuilder::default().build();

    camera
        .take_photo(world)
        .background(background)
        .height(432)
        .gamma(false)
        .samples(1)
        .shot(Some("rtow_6_1.ppm"))
        .unwrap();
}

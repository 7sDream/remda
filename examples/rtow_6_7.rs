use remda::{camera::CameraBuilder, hittable::collection::HittableList, prelude::*};

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> Option<(f64, Vec3)> {
    let oc = &ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = oc.dot(&ray.direction);
    let c = oc.length_squared() - radius * radius;
    #[allow(clippy::suspicious_operation_groupings)]
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        None
    } else {
        let t1 = (-half_b - discriminant.sqrt()) / a;
        let t2 = (-half_b + discriminant.sqrt()) / a;
        if t1 > 0.0 {
            let p = ray.position_after(t1);
            let n = (p - center) / radius;
            Some((t1, n))
        } else if t2 > 0.0 {
            let p = ray.position_after(t2);
            let n = (p - center) / radius;
            Some((t2, n))
        } else {
            None
        }
    }
}

fn background(ray: &Ray) -> Color {
    let ts = vec![
        hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray),
        hit_sphere(&Point3::new(0.0, -100.5, -1.0), 100.0, ray),
    ];
    if let Some((_, n)) = ts
        .into_iter()
        .flatten()
        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
    {
        return (0.5 * (n + Vec3::new(1.0, 1.0, 1.0))).into_color(1, false);
    }
    let unit = ray.direction.unit();
    let t = 0.5 * (unit.y + 1.0);
    Color::new(1.0, 1.0, 1.0).gradient(&Color::new(0.5, 0.7, 1.0), t)
}

fn main() {
    env_logger::init();

    let world = HittableList::default();

    let camera = CameraBuilder::default().aspect_ratio(2.0).build();

    camera
        .take_photo(world)
        .background(background)
        .height(100)
        .gamma(false)
        .samples(1)
        .shot(Some("rtow_6_7.ppm"))
        .unwrap();
}

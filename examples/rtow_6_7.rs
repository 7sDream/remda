use remda::{camera::CameraBuilder, geometry::World, prelude::*};

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> Option<(f64, Vec3)> {
    let oc = &ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = oc.dot(&ray.direction);
    let c = oc.length_squared() - radius * radius;
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

fn main() {
    env_logger::init();

    let mut world = World::default();
    world.set_bg(|ray| {
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
        Color::newf(1.0, 1.0, 1.0).gradient(&Color::newf(0.5, 0.7, 1.0), t)
    });

    let camera = CameraBuilder::default().aspect_ratio(2.0).build();

    camera
        .take_photo(&world)
        .height(100)
        .gamma(false)
        .samples(1)
        .shot(Some("rtow_6_7.ppm"))
        .unwrap();
}

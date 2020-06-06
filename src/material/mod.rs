use crate::{
    geometry::{Geometry, World},
    prelude::*,
};

fn normal_color(normal: &Vec3) -> Vec3 {
    (normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5
}

fn background(r: &Ray) -> Vec3 {
    let unit = r.direction.unit();
    let t = 0.5 * (unit.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

pub fn diffuse(r: &Ray, world: &World, depth: u64) -> Vec3 {
    if depth == 0 {
        Vec3::default()
    } else if let Some(record) = world.hit(r, 0.001..std::f64::INFINITY) {
        let dir = Vec3::random_unit_dir(&record.normal);
        let new_ray = Ray::new(record.point, dir);
        0.5 * diffuse(&new_ray, world, depth - 1)
    } else {
        background(r)
    }
}

#![deny(warnings)]
#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(missing_debug_implementations, rust_2018_idioms)]
#![allow(clippy::module_name_repetitions)]
#![allow(dead_code)]

use env_logger;

mod geometry;
mod image;
mod prelude;

use {
    geometry::{Geometry, Sphere},
    image::Painter,
    prelude::*,
};

fn ray_color(r: &Ray) -> Color {
    let unit = r.direction.unit();
    let t = 0.5 * (unit.y + 1.0);
    let c = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    c.into()
}

fn main() {
    env_logger::init();

    let aspect_radio = 16.0 / 9.0;
    let image_width = 384usize;
    let image_height = (image_width as f64 / aspect_radio) as usize;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_radio;
    let focal_length = 1.0;
    let origin = Point3::default();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lb = &origin - &horizontal / 2.0 - &vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    let painter = Painter::new(image_width, image_height);
    let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    painter
        .draw("first.ppm", |row, col| {
            let u = col as f64 / (image_width - 1) as f64;
            let v = (image_height - 1 - row) as f64 / (image_height - 1) as f64;
            let r = Ray::new(origin.clone(), &lb + &horizontal * u + &vertical * v - &origin);
            if let Some(record) = sphere.hit(&r, 0.0..f64::INFINITY) {
                ((record.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5).into()
            } else {
                ray_color(&r)
            }
        })
        .unwrap();
}

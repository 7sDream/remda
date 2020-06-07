use {crate::image::Painter, crate::prelude::*};

pub struct Camera {
    origin: Point3,
    lb: Point3,
    horizontal_full: Vec3,
    vertical_full: Vec3,
    horizontal_unit: Vec3,
    vertical_unit: Vec3,
    aspect_ratio: f64,
    aperture: f64,
    focus_distance: f64,
    samples: usize,
}

const FOCAL_LENGTH: f64 = 1.0;

impl Camera {
    pub fn new(
        look_from: Point3, look_at: Point3, vup: Vec3, fov: f64, aspect_ratio: f64, aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let fov = d2r(fov);
        let h = (fov / 2.0).tan();
        let vh = 2.0 * h;
        let vw = vh * aspect_ratio;

        let w = (&look_at - &look_from).unit();
        let horizontal_unit = w.cross(&vup).unit();
        let vertical_unit = horizontal_unit.cross(&w).unit();

        let horizontal_full = focus_distance * vw * &horizontal_unit;
        let vertical_full = focus_distance * vh * &vertical_unit;
        let lb = &look_from - &horizontal_full / 2.0 - &vertical_full / 2.0 + focus_distance * w;
        Self {
            origin: look_from,
            lb,
            horizontal_full,
            vertical_full,
            horizontal_unit,
            vertical_unit,
            aspect_ratio,
            aperture,
            focus_distance,
            samples: 1,
        }
    }

    pub fn set_samples(&mut self, samples: usize) {
        self.samples = samples;
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.aperture / 2.0 * Vec3::random_unit_disk();
        let offset = &self.horizontal_unit * rd.x + &self.vertical_unit * rd.y;
        let origin = &self.origin + offset;
        let direction = &self.lb + u * &self.horizontal_full + v * &self.vertical_full - &origin;

        Ray::new(origin, direction)
    }

    pub fn painter(&self, height: usize) -> Painter {
        let width = (height as f64 * self.aspect_ratio) as usize;
        Painter::new(width, height)
    }
}

use {crate::image::Painter, crate::prelude::*};

pub struct Camera {
    origin: Point3,
    lb: Point3,
    hor: Vec3,
    ver: Vec3,
    samples: usize,
}

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
const FOCAL_LENGTH: f64 = 1.0;

impl Camera {
    pub fn new(origin: Point3) -> Self {
        let hor = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let ver = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let lb = &origin - &hor / 2.0 - &ver / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);
        Self { origin, lb, hor, ver, samples: 1 }
    }

    pub fn set_samples(&mut self, samples: usize) {
        self.samples = samples;
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin.clone(), &self.lb + u * &self.hor + v * &self.ver - &self.origin)
    }

    pub fn painter(&self, width: usize) -> Painter {
        let height = (width as f64 / ASPECT_RATIO) as usize;
        Painter::new(width, height)
    }
}

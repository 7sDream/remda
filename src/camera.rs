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

pub struct CameraBuilder {
    from: Point3,
    to: Point3,
    vup: Vec3,
    fov: f64,
    aspect_ratio: f64,
    aperture: f64,
    focus_distance: f64,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            from: Point3::default(),
            to: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            fov: 90.0,
            aspect_ratio: 16.0 / 9.0,
            aperture: 0.0,
            focus_distance: 1.0,
        }
    }
}

impl CameraBuilder {
    pub fn look_from(mut self, from: Point3) -> Self {
        self.from = from;
        self
    }

    pub fn look_at(mut self, to: Point3) -> Self {
        self.to = to;
        self
    }

    pub fn vup(mut self, vup: Vec3) -> Self {
        self.vup = vup;
        self
    }

    pub fn fov(mut self, fov: f64) -> Self {
        debug_assert!(0.0 < fov && fov <= 180.0, "fov = {}", fov);
        self.fov = fov;
        self
    }

    pub fn aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        debug_assert!(aspect_ratio > 0.0, "aspect_ratio = {}", aspect_ratio);
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn aperture(mut self, aperture: f64) -> Self {
        debug_assert!(aperture >= 0.0, "aperture = {}", aperture);
        self.aperture = aperture;
        self
    }

    pub fn focus(mut self, distance: f64) -> Self {
        debug_assert!(distance >= 0.0, "distance = {}", distance);
        self.focus_distance = distance;
        self
    }

    pub fn focus_to_look_at(self) -> Self {
        let distance = (&self.to - &self.from).length();
        self.focus(distance)
    }

    pub fn build(self) -> Camera {
        Camera::new(self.from, self.to, self.vup, self.fov, self.aspect_ratio, self.aperture, self.focus_distance)
    }
}
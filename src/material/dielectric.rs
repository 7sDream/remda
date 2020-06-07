use {
    super::{Material, ScatterRecord},
    crate::{geometry::HitRecord, prelude::*},
};

pub trait ReflectProbabilityCurve {
    fn reflect_prob(&self, cos_theta: f64, refractive: f64) -> f64;
}

pub struct Glass {}

impl ReflectProbabilityCurve for Glass {
    fn reflect_prob(&self, cos_theta: f64, refractive: f64) -> f64 {
        let r0 = (1.0 - refractive) / (1.0 + refractive);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
    }
}

pub struct Dielectric<R> {
    color: Color,
    enter_refractive: f64,
    outer_refractive: f64,
    reflect_curve: R,
}

impl<R> Dielectric<R>
where
    R: ReflectProbabilityCurve,
{
    pub fn new(color: Color, refractive: f64, reflect_curve: R) -> Self {
        let enter_refractive = 1.0 / refractive;
        let outer_refractive = refractive;
        Self {
            color,
            enter_refractive,
            outer_refractive,
            reflect_curve,
        }
    }

    fn refract(&self, ray: &Ray, hit: &HitRecord) -> Option<Ray> {
        let dir = ray.direction.unit();
        let cos_theta = (-&dir).dot(&hit.normal);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let refractive = if hit.outside {
            self.enter_refractive
        } else {
            self.outer_refractive
        };
        if refractive * sin_theta > 1.0 {
            None
        } else if Random::normal() < self.reflect_curve.reflect_prob(cos_theta, refractive) {
            None
        } else {
            let r_parallel = refractive * (&dir + cos_theta * &hit.normal);
            let r_perp = -(1.0 - r_parallel.length_squared()).sqrt() * &hit.normal;
            let r = r_parallel + r_perp;
            Some(Ray::new(hit.point.clone(), r))
        }
    }

    fn reflect(&self, ray: &Ray, hit: &HitRecord) -> Ray {
        let dir = ray.direction.unit();
        let reflected_dir = &dir - 2.0 * dir.dot(&hit.normal) * &hit.normal;
        Ray::new(hit.point.clone(), reflected_dir)
    }
}

impl<R> Material for Dielectric<R>
where
    R: ReflectProbabilityCurve,
{
    fn scatter(&self, ray: &Ray, hit: HitRecord) -> Option<ScatterRecord> {
        let refract = self
            .refract(ray, &hit)
            .unwrap_or_else(|| self.reflect(ray, &hit));
        Some(ScatterRecord {
            color: self.color.clone(),
            ray: refract,
        })
    }
}

use {
    crate::{
        geometry::HitRecord,
        material::{reflect, Material, ScatterRecord},
        prelude::*,
    },
    std::fmt::Debug,
};

pub trait ReflectProbabilityCurve: Debug + Send + Sync {
    fn reflect_prob(&self, cos_theta: f64, refractive: f64) -> f64;
}

#[derive(Debug)]
pub struct Glass {}

impl ReflectProbabilityCurve for Glass {
    fn reflect_prob(&self, cos_theta: f64, refractive: f64) -> f64 {
        let r0 = (1.0 - refractive) / (1.0 + refractive);
        let r0 = r0 * r0;
        (1.0 - r0).mul_add((1.0 - cos_theta).powi(5), r0)
    }
}

#[derive(Debug)]
pub struct Dielectric {
    color: Color,
    enter_refractive: f64,
    outer_refractive: f64,
    reflect_curve: Option<Box<dyn ReflectProbabilityCurve>>,
}

impl Dielectric {
    #[must_use]
    pub fn new(color: Color, refractive: f64) -> Self {
        let enter_refractive = 1.0 / refractive;
        let outer_refractive = refractive;
        Self {
            color,
            enter_refractive,
            outer_refractive,
            reflect_curve: None,
        }
    }

    pub fn reflect_curve<R: ReflectProbabilityCurve + 'static>(mut self, reflect_curve: R) -> Self {
        self.reflect_curve = Some(Box::new(reflect_curve));
        self
    }

    fn refract(&self, ray: &Ray, hit: &HitRecord<'_>) -> Option<Ray> {
        let dir = ray.direction.unit();
        let cos_theta = (-&dir).dot(&hit.normal);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let refractive = if hit.outside {
            self.enter_refractive
        } else {
            self.outer_refractive
        };
        if refractive * sin_theta > 1.0 {
            return None;
        }
        let reflect_prob = self
            .reflect_curve
            .as_ref()
            .map_or(0.0, |r| r.reflect_prob(cos_theta, refractive));
        if Random::normal() < reflect_prob {
            return None;
        }
        let r_parallel = refractive * (&dir + cos_theta * &hit.normal);
        let r_perpendicular = -(1.0 - r_parallel.length_squared()).sqrt() * &hit.normal;
        let r = r_parallel + r_perpendicular;
        Some(Ray::new(hit.point.clone(), r, ray.departure_time))
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: HitRecord<'_>) -> Option<ScatterRecord> {
        let refract = self
            .refract(ray, &hit)
            .unwrap_or_else(|| reflect(ray, &hit));
        Some(ScatterRecord {
            color: self.color.clone(),
            ray: refract,
        })
    }
}

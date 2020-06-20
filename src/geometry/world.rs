use crate::geometry::AABB;
use {
    super::{Geometry, HitRecord},
    crate::{material::Material, prelude::*},
    std::{
        fmt::{Debug, Formatter},
        ops::Range,
    },
};

#[derive(Default)]
pub struct World {
    background: Option<Box<dyn Fn(&Ray) -> Color + Send + Sync>>,
    objects: Vec<Box<dyn Geometry>>,
}

impl Debug for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("World {{ objects: {}}}", self.objects.len()))
    }
}

impl World {
    pub fn add<G: Geometry + 'static>(&mut self, object: G) -> &mut Self {
        let object: Box<dyn Geometry> = Box::new(object);
        self.objects.push(object);
        self
    }

    pub fn add_ref(&mut self, object: Box<dyn Geometry>) -> &mut Self {
        self.objects.push(object);
        self
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn set_bg<F>(&mut self, f: F)
    where
        F: Fn(&Ray) -> Color + Send + Sync + 'static,
    {
        self.background = Some(Box::new(f));
    }

    #[must_use]
    pub fn background(&self, ray: &Ray) -> Color {
        if let Some(f) = &self.background {
            f(ray)
        } else {
            Color::default()
        }
    }

    pub fn into_vec(self) -> Vec<Box<dyn Geometry>> {
        self.objects
    }
}

impl Geometry for World {
    fn normal(&self, _p: &Point3) -> Vec3 {
        unimplemented!("World's normal function should not be called directly")
    }

    fn material(&self) -> &dyn Material {
        unimplemented!("World's material function should not be called directly")
    }

    fn hit(&self, r: &Ray, limit: Range<f64>) -> Option<HitRecord<'_>> {
        self.objects
            .iter()
            .filter_map(|object| object.hit(r, limit.clone()))
            .min_by(|r1, r2| r1.t.partial_cmp(&r2.t).unwrap())
    }

    fn bbox(&self, limit: Range<f64>) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut result: Option<AABB> = None;

        for object in &self.objects {
            let bbox = object.bbox(limit.clone())?;
            result = result.map(|last| last | &bbox).or_else(|| Some(bbox))
        }

        result
    }
}

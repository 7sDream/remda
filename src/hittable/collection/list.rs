use {
    crate::{
        hittable::{Hittable, HitRecord},
        prelude::*,
    },
    std::{
        fmt::{Debug, Formatter},
        ops::Range,
    },
};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl Debug for HittableList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "GeometryList {{ objects: {}}}",
            self.objects.len()
        ))
    }
}

impl HittableList {
    pub fn add<G: Hittable + 'static>(&mut self, object: G) -> &mut Self {
        let object: Box<dyn Hittable> = Box::new(object);
        self.objects.push(object);
        self
    }

    pub fn add_ref(&mut self, object: Box<dyn Hittable>) -> &mut Self {
        self.objects.push(object);
        self
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    #[must_use]
    pub fn into_objects(self) -> Vec<Box<dyn Hittable>> {
        self.objects
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, unit_limit: Range<f64>) -> Option<HitRecord<'_>> {
        self.objects
            .iter()
            .filter_map(|object| object.hit(r, unit_limit.clone()))
            .min_by(|r1, r2| r1.unit.partial_cmp(&r2.unit).unwrap())
    }

    fn bbox(&self, time_limit: Range<f64>) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut result: Option<AABB> = None;

        for object in &self.objects {
            let bbox = object.bbox(time_limit.clone())?;
            result = result.map(|last| last | &bbox).or(Some(bbox))
        }

        result
    }
}

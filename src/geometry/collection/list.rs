use {
    crate::{
        geometry::{Geometry, HitRecord},
        prelude::*,
    },
    std::{
        fmt::{Debug, Formatter},
        ops::Range,
    },
};

#[derive(Default)]
pub struct GeometryList {
    objects: Vec<Box<dyn Geometry>>,
}

impl Debug for GeometryList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "GeometryList {{ objects: {}}}",
            self.objects.len()
        ))
    }
}

impl GeometryList {
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

    #[must_use]
    pub fn into_objects(self) -> Vec<Box<dyn Geometry>> {
        self.objects
    }
}

impl Geometry for GeometryList {
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

use {
    super::{Geometry, HitRecord},
    crate::{material::Material, prelude::*},
    std::{
        fmt::{Debug, Formatter},
        ops::Range,
        sync::Arc,
    },
};

#[derive(Default)]
pub struct World {
    objects: Vec<Arc<dyn Geometry>>,
}

impl Debug for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("World {{ objects: {}}}", self.objects.len()))
    }
}

impl World {
    pub fn add<G: Geometry + 'static>(&mut self, object: G) -> &mut Self {
        let object: Arc<dyn Geometry> = Arc::new(object);
        self.objects.push(object);
        self
    }

    pub fn add_ref(&mut self, object: Arc<dyn Geometry>) -> &mut Self {
        self.objects.push(object);
        self
    }

    pub fn clear(&mut self) {
        self.objects.clear();
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
}

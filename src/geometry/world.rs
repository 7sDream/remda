use {
    super::{Geometry, HitRecord},
    crate::{material::Material, prelude::*},
    std::{
        fmt::{Debug, Formatter},
        ops::Range,
        rc::Rc,
    },
};

#[derive(Default)]
pub struct World {
    objects: Vec<Rc<dyn Geometry>>,
}

impl Debug for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("World {{ objects: {}}}", self.objects.len()))
    }
}

impl World {
    pub fn add<G: Geometry + 'static>(&mut self, object: G) -> &mut Self {
        let object: Rc<dyn Geometry> = Rc::new(object);
        self.objects.push(object);
        self
    }

    pub fn add_ref(&mut self, object: Rc<dyn Geometry>) -> &mut Self {
        self.objects.push(object);
        self
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Geometry for World {
    fn normal(&self, _p: &Point3) -> crate::prelude::Vec3 {
        unimplemented!("World's normal function should not be called directly")
    }

    fn material(&self) -> Rc<dyn Material> {
        unimplemented!("World's material function should not be called directly")
    }

    fn hit(&self, r: &Ray, limit: Range<f64>) -> Option<HitRecord> {
        self.objects
            .iter()
            .filter_map(|object| object.hit(r, limit.clone()))
            .min_by(|r1, r2| r1.t.partial_cmp(&r2.t).unwrap())
    }
}

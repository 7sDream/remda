use {
    super::{AARect, AARectGeometry, Geometry, GeometryList, HitRecord, AABB},
    crate::{material::Material, prelude::*},
    std::{ops::Range, sync::Arc},
};

#[derive(Debug)]
pub struct Carton {
    faces: GeometryList,
}

impl Carton {
    pub fn new<M: Material + 'static>(p0: &Point3, p1: &Point3, material: M) -> Self {
        let pmin = Point3::new_min(p0, p1);
        let pmax = Point3::new_max(p0, p1);
        let m = Arc::new(material);
        let mut faces = GeometryList::default();
        faces
            .add(AARect::new_xy(
                // back
                AARectGeometry::new(pmin.z, (pmin.x, pmax.x), (pmin.y, pmax.y)),
                Arc::clone(&m),
            ))
            .add(AARect::new_xy(
                // front
                AARectGeometry::new(pmax.z, (pmin.x, pmax.x), (pmin.y, pmax.y)),
                Arc::clone(&m),
            ))
            .add(AARect::new_yz(
                // left
                AARectGeometry::new(pmin.x, (pmin.y, pmax.y), (pmin.z, pmax.z)),
                Arc::clone(&m),
            ))
            .add(AARect::new_yz(
                // right
                AARectGeometry::new(pmax.x, (pmin.y, pmax.y), (pmin.z, pmax.z)),
                Arc::clone(&m),
            ))
            .add(AARect::new_xz(
                // down
                AARectGeometry::new(pmin.y, (pmin.x, pmax.x), (pmin.z, pmax.z)),
                Arc::clone(&m),
            ))
            .add(AARect::new_xz(
                // up
                AARectGeometry::new(pmax.y, (pmin.x, pmax.x), (pmin.z, pmax.z)),
                Arc::clone(&m),
            ));

        Self { faces }
    }
}

impl Geometry for Carton {
    fn hit(&self, ray: &Ray, unit_limit: Range<f64>) -> Option<HitRecord<'_>> {
        self.faces.hit(ray, unit_limit)
    }

    fn bbox(&self, time_limit: Range<f64>) -> Option<AABB> {
        self.faces.bbox(time_limit)
    }
}

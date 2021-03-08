use {
    super::{AARect, AARectGeometry, Geometry, GeometryList, HitRecord, AABB},
    crate::{material::Material, prelude::*},
    std::{ops::Range, sync::Arc},
};

#[derive(Debug)]
pub struct Carton<M> {
    min: Point3,
    max: Point3,
    material: Arc<M>,
    faces: GeometryList,
}

impl<M: Material + 'static> Clone for Carton<M> {
    fn clone(&self) -> Self {
        Self::new_inner(
            self.min.clone(),
            self.max.clone(),
            Arc::clone(&self.material),
        )
    }
}

impl<M: Material + 'static> Carton<M> {
    pub fn new(p0: Point3, p1: Point3, material: M) -> Self {
        let pmin = Point3::new_min(&p0, &p1);
        let pmax = Point3::new_max(&p0, &p1);
        let shared_material = Arc::new(material);
        Self::new_inner(pmin, pmax, shared_material)
    }

    fn new_inner(pmin: Point3, pmax: Point3, material: Arc<M>) -> Self {
        let mut faces = GeometryList::default();
        faces
            .add(AARect::new_xy(
                // back
                AARectGeometry::new(pmin.z, (pmin.x, pmax.x), (pmin.y, pmax.y)),
                Arc::clone(&material),
            ))
            .add(AARect::new_xy(
                // front
                AARectGeometry::new(pmax.z, (pmin.x, pmax.x), (pmin.y, pmax.y)),
                Arc::clone(&material),
            ))
            .add(AARect::new_yz(
                // left
                AARectGeometry::new(pmin.x, (pmin.y, pmax.y), (pmin.z, pmax.z)),
                Arc::clone(&material),
            ))
            .add(AARect::new_yz(
                // right
                AARectGeometry::new(pmax.x, (pmin.y, pmax.y), (pmin.z, pmax.z)),
                Arc::clone(&material),
            ))
            .add(AARect::new_xz(
                // down
                AARectGeometry::new(pmin.y, (pmin.x, pmax.x), (pmin.z, pmax.z)),
                Arc::clone(&material),
            ))
            .add(AARect::new_xz(
                // up
                AARectGeometry::new(pmax.y, (pmin.x, pmax.x), (pmin.z, pmax.z)),
                Arc::clone(&material),
            ));

        Self {
            min: pmin,
            max: pmax,
            material,
            faces,
        }
    }
}

impl<M: Material> Geometry for Carton<M> {
    fn hit(&self, ray: &Ray, unit_limit: Range<f64>) -> Option<HitRecord<'_>> {
        self.faces.hit(ray, unit_limit)
    }

    fn bbox(&self, time_limit: Range<f64>) -> Option<AABB> {
        self.faces.bbox(time_limit)
    }
}

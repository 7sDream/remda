use {
    crate::{
        geometry::{collection::GeometryList, AARect, AARectMetrics, Geometry, HitRecord},
        material::Material,
        prelude::*,
    },
    std::{ops::Range, sync::Arc},
};

#[derive(Debug)]
pub struct Carton<M> {
    point_min: Point3,
    point_max: Point3,
    material: Arc<M>,
    faces: GeometryList,
}

impl<M: Material + 'static> Clone for Carton<M> {
    fn clone(&self) -> Self {
        Self::new_inner(
            self.point_min.clone(),
            self.point_max.clone(),
            Arc::clone(&self.material),
        )
    }
}

impl<M: Material + 'static> Carton<M> {
    #[allow(clippy::needless_pass_by_value)] // for api consistency
    pub fn new(p0: Point3, p1: Point3, material: M) -> Self {
        let point_min = Point3::new_min(&p0, &p1);
        let point_max = Point3::new_max(&p0, &p1);
        let shared_material = Arc::new(material);
        Self::new_inner(point_min, point_max, shared_material)
    }

    #[allow(clippy::too_many_lines)]
    fn new_inner(point_min: Point3, point_max: Point3, material: Arc<M>) -> Self {
        let mut faces = GeometryList::default();
        faces
            .add(AARect::new_xy(
                // back
                AARectMetrics::new(
                    point_min.z,
                    (point_min.x, point_max.x),
                    (point_min.y, point_max.y),
                ),
                Arc::clone(&material),
            ))
            .add(AARect::new_xy(
                // front
                AARectMetrics::new(
                    point_max.z,
                    (point_min.x, point_max.x),
                    (point_min.y, point_max.y),
                ),
                Arc::clone(&material),
            ))
            .add(AARect::new_yz(
                // left
                AARectMetrics::new(
                    point_min.x,
                    (point_min.y, point_max.y),
                    (point_min.z, point_max.z),
                ),
                Arc::clone(&material),
            ))
            .add(AARect::new_yz(
                // right
                AARectMetrics::new(
                    point_max.x,
                    (point_min.y, point_max.y),
                    (point_min.z, point_max.z),
                ),
                Arc::clone(&material),
            ))
            .add(AARect::new_xz(
                // down
                AARectMetrics::new(
                    point_min.y,
                    (point_min.x, point_max.x),
                    (point_min.z, point_max.z),
                ),
                Arc::clone(&material),
            ))
            .add(AARect::new_xz(
                // up
                AARectMetrics::new(
                    point_max.y,
                    (point_min.x, point_max.x),
                    (point_min.z, point_max.z),
                ),
                Arc::clone(&material),
            ));

        Self {
            point_min,
            point_max,
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

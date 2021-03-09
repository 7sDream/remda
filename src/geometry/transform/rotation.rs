use {
    crate::{
        geometry::{Geometry, HitRecord, AABB},
        prelude::*,
    },
    once_cell::sync::OnceCell,
    std::marker::PhantomData,
};

pub trait RotationByAxes: Send + Sync {
    fn rotate(point: &Point3, radian: f64) -> Point3;
}

#[derive(Debug)]
pub struct ByYAxes;

impl RotationByAxes for ByYAxes {
    fn rotate(point: &Point3, radian: f64) -> Point3 {
        Point3::new(
            radian.cos() * point.x + radian.sin() * point.z,
            point.y,
            -radian.sin() * point.x + radian.cos() * point.z,
        )
    }
}

#[derive(Debug)]
pub struct AARotation<Axes, G> {
    geometry: G,
    radian: f64,
    bbox_cache: OnceCell<Option<AABB>>,
    axes: PhantomData<Axes>,
}

impl<G> AARotation<ByYAxes, G> {
    pub fn new_by_y(geometry: G, angle: f64) -> Self {
        Self {
            geometry,
            radian: d2r(angle),
            bbox_cache: OnceCell::new(),
            axes: PhantomData,
        }
    }
}

impl<Axes: RotationByAxes, G: Geometry> Geometry for AARotation<Axes, G> {
    fn hit(&self, ray: &Ray, unit_limit: std::ops::Range<f64>) -> Option<HitRecord<'_>> {
        let rotated_origin = Axes::rotate(&ray.origin, -self.radian);
        let rotated_direction = Axes::rotate(&ray.direction, -self.radian);
        let rotated_ray = Ray::new(rotated_origin, rotated_direction, ray.departure_time);
        self.geometry
            .hit(&rotated_ray, unit_limit)
            .map(|mut record| {
                record.point = Axes::rotate(&record.point, self.radian);
                record.normal = Axes::rotate(&record.normal, self.radian);
                record
            })
    }

    fn bbox(&self, time_limit: std::ops::Range<f64>) -> Option<AABB> {
        self.bbox_cache
            .get_or_init(|| {
                self.geometry.bbox(time_limit).map(|bbox| {
                    let mut point_min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
                    let mut point_max =
                        Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

                    for i in 0..2_usize {
                        for j in 0..2_usize {
                            for k in 0..2_usize {
                                let x = i as f64 * bbox.max().x + (1 - i) as f64 * bbox.min().x;
                                let y = j as f64 * bbox.max().y + (1 - j) as f64 * bbox.min().y;
                                let z = k as f64 * bbox.max().z + (1 - k) as f64 * bbox.min().z;

                                let rotated_point =
                                    Axes::rotate(&Point3::new(x, y, z), self.radian);

                                for c in 0..3 {
                                    point_min[c] = point_min[c].min(rotated_point[c]);
                                    point_max[c] = point_max[c].max(rotated_point[c]);
                                }
                            }
                        }
                    }

                    AABB::new(point_min, point_max)
                })
            })
            .clone()
    }
}

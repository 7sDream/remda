use {
    super::{Geometry, AABB},
    crate::{geometry::HitRecord, material::Material, prelude::*},
    std::{
        cmp::Ordering,
        fmt::{Debug, Formatter},
        ops::Range,
    },
};

pub struct BVH {
    bbox: AABB,
    left: Box<dyn Geometry>,
    right: Option<Box<dyn Geometry>>,
}

impl Debug for BVH {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("BVH {{ bbox: {:?} }}", self.bbox))
    }
}

fn cmp_geometry_by(axis: usize, a: &dyn Geometry, b: &dyn Geometry) -> Ordering {
    let box_a = a
        .bbox(0.0..0.0)
        .expect("No bounding box in bvh_node constructor");
    let box_b = b
        .bbox(0.0..0.0)
        .expect("No bounding box in bvh_node constructor");

    box_a.min()[axis]
        .partial_cmp(&box_b.min()[axis])
        .expect("Bounding box contains NaN")
}

impl BVH {
    #[must_use]
    pub fn new(objects: Vec<Box<dyn Geometry>>, limit: Range<f64>) -> Self {
        let mut objects: Vec<_> = objects.into_iter().map(Some).collect();
        let count = objects.len();
        Self::new_internal(&mut objects, 0..count, limit)
    }

    fn new_internal(
        objects: &mut Vec<Option<Box<dyn Geometry>>>, index: Range<usize>, limit: Range<f64>,
    ) -> Self {
        let count = index.end - index.start;

        if count == 1 {
            let left = objects[index.start].take().unwrap();
            let bbox = left
                .bbox(limit)
                .expect("No bounding box in bvh_node constructor.");
            Self {
                bbox,
                left,
                right: None,
            }
        } else if count == 2 {
            let left = objects[index.start].take().unwrap();
            let right = objects[index.start + 1].take().unwrap();
            let left_bbox = left
                .bbox(limit.clone())
                .expect("No bounding box in bvh_node constructor.");
            let right_bbox = right
                .bbox(limit)
                .expect("No bounding box in bvh_node constructor.");
            Self {
                bbox: left_bbox | right_bbox,
                left,
                right: Some(right),
            }
        } else {
            let axis = *Random::choose(&[0, 1, 2]);
            objects[index.clone()].sort_by(|a, b| {
                cmp_geometry_by(
                    axis,
                    a.as_ref().unwrap().as_ref(),
                    b.as_ref().unwrap().as_ref(),
                )
            });
            let mid = index.start + count / 2;
            let left = Box::new(Self::new_internal(objects, index.start..mid, limit.clone()));
            let right = Box::new(Self::new_internal(objects, mid..index.end, limit));
            Self {
                bbox: &left.bbox | &right.bbox,
                left,
                right: Some(right),
            }
        }
    }
}

/// Bounding Volume Hierarchies
impl Geometry for BVH {
    fn normal(&self, _p: &Point3) -> Vec3 {
        unimplemented!("BVH's normal function should not be called directly")
    }

    fn material(&self) -> &dyn Material {
        unimplemented!("BVH's material function should not be called directly")
    }

    fn hit(&self, ray: &Ray, limit: Range<f64>) -> Option<HitRecord<'_>> {
        if !self.bbox.hit(ray, limit.clone()) {
            return None;
        }

        let hit_left = self.left.hit(ray, limit.clone());
        let hit_right = self.right.as_ref().and_then(|right| {
            let right_limit = limit.start..hit_left.as_ref().map_or(limit.end, |record| record.t);
            right.hit(ray, right_limit)
        });

        // Right has small t then left if it return `Some`, so right appear first
        hit_right.or(hit_left)
    }

    fn bbox(&self, _limit: Range<f64>) -> Option<AABB> {
        Some(self.bbox.clone())
    }
}

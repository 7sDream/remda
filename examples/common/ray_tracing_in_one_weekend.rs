use {
    super::common,
    remda::{camera::Camera, geometry::GeometryList},
};

#[must_use]
fn final_world(seed: Option<u64>) -> GeometryList {
    common::world(seed, false, false)
}

#[must_use]
fn final_camera() -> Camera {
    common::camera(false)
}

#[must_use]
pub fn final_scene(seed: Option<u64>) -> (Camera, GeometryList) {
    (final_camera(), final_world(seed))
}

use {
    super::common,
    remda::{camera::Camera, geometry::GeometryList},
};

#[must_use]
fn motion_blur_world(seed: Option<u64>) -> GeometryList {
    common::world(seed, true)
}

#[must_use]
fn motion_blur_camera() -> Camera {
    common::camera(true)
}

#[must_use]
pub fn motion_blur(seed: Option<u64>) -> (Camera, GeometryList) {
    (motion_blur_camera(), motion_blur_world(seed))
}

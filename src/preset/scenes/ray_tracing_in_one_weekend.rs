use {
    super::common,
    crate::{camera::Camera, geometry::World},
};

#[must_use]
fn final_world(seed: Option<u64>) -> World {
    common::world(seed, false)
}

#[must_use]
fn final_camera() -> Camera {
    common::camera(false)
}

#[must_use]
pub fn final_scene(seed: Option<u64>) -> (Camera, World) {
    (final_camera(), final_world(seed))
}

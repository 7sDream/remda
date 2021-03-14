use {
    super::scene,
    remda::{camera::Camera, hittable::collection::HittableList},
};

#[must_use]
fn final_world(seed: Option<u64>) -> HittableList {
    scene::balls_scene(seed, false, false)
}

#[must_use]
fn final_camera() -> Camera {
    scene::balls_scene_camera(false)
}

#[must_use]
pub fn final_scene(seed: Option<u64>) -> (Camera, HittableList) {
    (final_camera(), final_world(seed))
}

use {
    super::scene,
    remda::{camera::Camera, hittable::collection::HittableList},
};

pub use scene::all_feature_scene;

#[must_use]
fn motion_blur_world(seed: Option<u64>, checker: bool) -> HittableList {
    scene::balls_scene(seed, true, checker)
}

#[must_use]
fn motion_blur_camera() -> Camera {
    scene::balls_scene_camera(true)
}

#[must_use]
pub fn motion_blur(seed: Option<u64>, checker: bool) -> (Camera, HittableList) {
    (motion_blur_camera(), motion_blur_world(seed, checker))
}

#[must_use]
pub fn empty_cornell_box() -> (Camera, HittableList) {
    scene::cornell_box_scene(false, false, false)
}

#[must_use]
pub fn cornell_box_no_rotation() -> (Camera, HittableList) {
    scene::cornell_box_scene(true, false, false)
}

#[must_use]
pub fn cornell_box() -> (Camera, HittableList) {
    scene::cornell_box_scene(true, true, false)
}

#[must_use]
pub fn cornell_box_smoke() -> (Camera, HittableList) {
    scene::cornell_box_scene(true, true, true)
}

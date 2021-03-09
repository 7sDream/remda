#[allow(dead_code)]
mod common;

fn main() {
    common::init_log("info");

    // Change `7` to another number to generate different scene
    // Or use `None` to use random seed
    let (camera, world) = common::ray_tracing_in_one_weekend::final_scene(Some(7));

    camera
        .take_photo(world)
        .height(108)
        .samples(128)
        .shot(Some("rtow_13_1.ppm"))
        .unwrap();
}

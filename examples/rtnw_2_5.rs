#[allow(dead_code)]
mod common;

fn main() {
    common::init_log("info");

    // Change `77` to another number to generate different scene
    // Or use `None` to use random seed
    let (camera, world) = common::ray_tracing_next_week::motion_blur(Some(77), false);

    camera
        .take_photo(world)
        .height(100)
        .samples(100)
        .shot(Some("rtnw_2_5.ppm"))
        .unwrap();
}

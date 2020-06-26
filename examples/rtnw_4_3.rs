#[allow(dead_code)]
mod common;

use common::ray_tracing_next_week::motion_blur as scene;

fn init_log(level: &'static str) {
    let env = env_logger::Env::default().default_filter_or(level);
    env_logger::init_from_env(env);
}

fn main() {
    init_log("info");

    // Change `77` to another number to generate different scene
    // Or use `None` to use random seed
    let (camera, world) = scene(Some(77), true);

    camera
        .take_photo(world)
        .height(500)
        .samples(128)
        .shot(Some("rtnw_4_3.ppm"))
        .unwrap();
}

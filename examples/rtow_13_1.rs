#[allow(dead_code)]
mod common;

fn init_log(level: &'static str) {
    let env = env_logger::Env::default().default_filter_or(level);
    env_logger::init_from_env(env);
}

fn main() {
    init_log("info");

    // Change `7` to another number to generate different scene
    // Or use `None` to use random seed
    let (camera, world) = common::ray_tracing_in_one_weekend::final_scene(Some(7));

    camera
        .take_photo(world)
        .height(1080)
        .samples(512)
        .shot(Some("rtow_13_1.ppm"))
        .unwrap();
}

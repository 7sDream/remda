#[allow(dead_code)]
mod common;

fn init_log(level: &'static str) {
    let env = env_logger::Env::default().default_filter_or(level);
    env_logger::init_from_env(env);
}

fn main() {
    init_log("info");

    // Change `77` to another number to generate different scene
    // Or use `None` to use random seed
    let (camera, world) = common::ray_tracing_next_week::motion_blur(Some(77));

    camera
        .take_photo(world)
        .height(100)
        .samples(100)
        .shot(Some("rtnw_2_5.ppm"))
        .unwrap();
}

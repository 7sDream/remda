#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(missing_debug_implementations, rust_2018_idioms)]
#![deny(warnings)]
#![allow(clippy::module_name_repetitions)]

use remda::preset::scenes::ray_tracing_in_one_weekend::final_scene as scene;

fn init_log(level: &'static str) {
    let env = env_logger::Env::default().default_filter_or(level);
    env_logger::init_from_env(env);
}

fn main() {
    init_log("info");

    let (camera, world) = scene(None);

    camera
        .take_photo(&world)
        .height(108)
        .samples(128)
        .shot(Some("rendered.ppm"))
        .unwrap();
}

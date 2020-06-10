use {
    std::time::Instant,
    remda::preset::scenes::ray_tracing_in_one_weekend as final_scene
};
use std::io::Write;

fn main() {
    let world = final_scene::world(None);
    let camera = final_scene::camera();

    let prepare = camera
        .take_photo(&world)
        .height(108)
        .samples(64);

    let start = Instant::now();
    prepare.shot::<&'static str>(None).unwrap();
    std::io::stdout().write_fmt(format_args!("Time usage: {:?}", start.elapsed())).unwrap();
    std::io::stdout().flush().unwrap();
}

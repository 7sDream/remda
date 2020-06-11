use {remda::preset::scenes::ray_tracing_in_one_weekend as final_scene, std::time::Instant};

fn main() {
    // Fixed random seed to ensure our benchmark is using identical scene
    let world = final_scene::world(Some(7u64));
    let camera = final_scene::camera();

    let prepare = camera.take_photo(&world).height(108).samples(128);

    let start = Instant::now();
    prepare.shot::<&'static str>(None).unwrap();
    println!("Time usage: {:?}", start.elapsed());
}

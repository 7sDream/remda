use {
    remda::preset::scenes::ray_tracing_in_one_weekend as final_scene,
    test::Bencher,
};

mod bench {
    #[bench]
    pub fn bench_whole_render_process(b: &mut Bencher) {
        let world = final_scene::world(None);
        let camera = final_scene::camera();

        let prepare = camera
            .take_photo(&world)
            .height(108)
            .samples(128);

        b.iter(|| prepare.shot::<&'static str>(None));
    }
}


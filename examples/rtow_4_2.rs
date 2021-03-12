use remda::{camera::CameraBuilder, hittable::collection::HittableList};

fn main() {
    env_logger::init();

    let world = HittableList::default();

    let camera = CameraBuilder::default().build();

    camera
        .take_photo(world)
        .height(432)
        .gamma(false)
        .samples(1)
        .shot(Some("rtow_4_2.ppm"))
        .unwrap();
}

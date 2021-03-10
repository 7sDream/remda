use remda::{camera::CameraBuilder, geometry::collection::GeometryList};

fn main() {
    env_logger::init();

    let world = GeometryList::default();

    let camera = CameraBuilder::default().build();

    camera
        .take_photo(world)
        .height(432)
        .gamma(false)
        .samples(1)
        .shot(Some("rtow_4_2.ppm"))
        .unwrap();
}

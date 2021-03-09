#[allow(dead_code)]
mod common;

use remda::prelude::*;

fn main() {
    common::init_log("info");

    let (camera, world) = common::ray_tracing_next_week::cornell_box(true, true);

    camera
        .take_photo(world)
        .background(|_| Color::default())
        .height(300)
        .depth(50)
        .samples(1000)
        .shot(Some("rtnw_8_2.ppm"))
        .unwrap();
}

#[allow(dead_code)]
mod common;

use remda::prelude::*;

fn main() {
    common::init_log("info");

    let (camera, world) = common::ray_tracing_next_week::cornell_box(true, true, true);

    camera
        .take_photo(world)
        .background(|_| Color::default())
        .height(1000)
        .depth(10)
        .samples(10240)
        .shot(Some("rtnw_9_2.ppm"))
        .unwrap();
}

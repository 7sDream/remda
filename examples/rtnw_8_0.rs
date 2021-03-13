#[allow(dead_code)]
mod common;

use remda::prelude::*;

fn main() {
    let (camera, world) = common::ray_tracing_next_week::cornell_box(true, false, false);

    camera
        .take_photo(world)
        .background(|_| Color::default())
        .height(300)
        .depth(50)
        .samples(1000)
        .shot(Some("rtnw_8_0.ppm"))
        .unwrap();
}

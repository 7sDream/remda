#[allow(dead_code)]
mod common;

use remda::prelude::*;

fn main() {
    let (camera, world) = common::ray_tracing_next_week::empty_cornell_box();

    camera
        .take_photo(world)
        .background(|_| Color::default())
        .height(300)
        .depth(50)
        .samples(1000)
        .shot(Some("rtnw_7_6.ppm"))
        .unwrap();
}

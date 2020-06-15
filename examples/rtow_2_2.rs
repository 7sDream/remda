use remda::{image::Painter, prelude::*};

fn main() {
    Painter::new(256, 256)
        .gamma(false)
        .samples(1)
        .draw(&Some("rtow_2_2.ppm"), |u, v| Vec3::new(u, v, 0.25))
        .unwrap()
}

#![deny(warnings)]
#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(missing_debug_implementations, rust_2018_idioms)]
#![allow(clippy::module_name_repetitions)]
#![allow(dead_code)]

use {env_logger, log::info, std::iter::from_fn};

mod ppm;

fn main() {
    env_logger::init();
    let mut col = 0usize;
    let mut row = 256usize;
    let mut picture: ppm::Image = from_fn(|| {
        if col == 256 {
            col = 0;
            row -= 1;
            info!("Scan lines remaining: {}", row);
        }
        if row == 0 {
            return None;
        }
        let c = ppm::Color::newf(col as f32 / 255.0, (row - 1) as f32 / 255.0, 0.25);
        col += 1;
        Some(c)
    })
    .collect();
    picture.reshape(256).unwrap();
    picture[(0, 0)] = ppm::BLACK.clone();
    picture.save("first.ppm").unwrap();
}

#![deny(warnings)]
#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(missing_debug_implementations, rust_2018_idioms)]
#![allow(clippy::module_name_repetitions)]
#![allow(dead_code)]

use env_logger;

mod color;
mod image;
mod ray;
mod vec3;

fn main() {
    env_logger::init();
    let painter = image::Painter::new(256, 256, "first.ppm").unwrap();
    painter.draw(|row, col| color::Color::new(col as u8, 255 - row as u8, 64)).unwrap();
}

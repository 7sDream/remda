use remda::prelude::*;

const N: usize = 1000;

fn main() {
    let mut inside_circle: u64 = 0;
    for _ in 0..N {
        let x: f64 = Random::range(-1.0..1.0);
        let y: f64 = Random::range(-1.0..1.0);
        if x.powi(2) + y.powi(2) <= 1.0 {
            inside_circle += 1;
        }
    }
    println!(
        "Estimate of Pi = {:.12}",
        4.0 * inside_circle as f64 / N as f64
    );
}

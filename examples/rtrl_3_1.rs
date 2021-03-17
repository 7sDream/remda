use remda::prelude::*;

const N: u64 = 1000000;

// Calculate \int_{0}^{2}x^2
// Which is 8/3

fn main() {
    let mut sum: f64 = 0.0;

    for _ in 0..N {
        sum += Random::range(0.0..2.0_f64).powi(2);
    }

    println!("I = {:.12}", 2.0 * sum / N as f64,);
}

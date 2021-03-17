use remda::prelude::*;

const N: u64 = 1000000;

// Calculate \int_{0}^{2}x^2
// Which is 8/3

fn pdf(x: f64) -> f64 {
    0.5 * x
}

// pdf = 0.5x
// P(x) = \int_{0}^{x} 0.5r dr = 1/4 * x^2
// We use x = \sqrt{4 * y} to generate x
// where y is uniform distribution in range [0, 1]
fn generate_x() -> f64 {
    Random::range(0.0..4.0f64).sqrt()
}

fn main() {
    let mut sum: f64 = 0.0;

    for _ in 0..N {
        let x = generate_x();
        sum += x * x / pdf(x);
    }

    println!("I = {:.12}", sum / N as f64,);
}

use remda::prelude::*;

fn main() {
    let mut runs: u64 = 0;
    let mut inside_circle: u64 = 0;
    loop {
        runs += 1;
        let x: f64 = Random::range(-1.0..1.0);
        let y: f64 = Random::range(-1.0..1.0);
        if x.powi(2) + y.powi(2) <= 1.0 {
            inside_circle += 1;
        }
        if runs % 100000 == 0 {
            println!(
                "Estimate of Pi = {:.12}",
                4.0 * inside_circle as f64 / runs as f64
            );
        }
    }
}

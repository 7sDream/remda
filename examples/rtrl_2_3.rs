use remda::prelude::*;

const GRID_LINE_PRE_SIDE: u64 = 10000;

fn main() {
    let mut inside_circle: u64 = 0;
    let mut inside_circle_stratified: u64 = 0;

    for i in 0..GRID_LINE_PRE_SIDE {
        for j in 0..GRID_LINE_PRE_SIDE {
            let x: f64 = Random::range(-1.0..1.0);
            let y: f64 = Random::range(-1.0..1.0);
            if x.powi(2) + y.powi(2) <= 1.0 {
                inside_circle += 1;
            }

            let x = -1.0 + 2.0 * (j as f64 + Random::normal()) / GRID_LINE_PRE_SIDE as f64;
            let y = -1.0 + 2.0 * (i as f64 + Random::normal()) / GRID_LINE_PRE_SIDE as f64;
            if x.powi(2) + y.powi(2) <= 1.0 {
                inside_circle_stratified += 1;
            }
        }
    }

    println!(
        "Regular    Estimate of Pi = {:.12}",
        4.0 * inside_circle as f64 / (GRID_LINE_PRE_SIDE as f64).powi(2)
    );
    println!(
        "Stratified Estimate of Pi = {:.12}",
        4.0 * inside_circle_stratified as f64 / (GRID_LINE_PRE_SIDE as f64).powi(2)
    );
}

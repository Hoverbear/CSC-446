extern crate libc;
use libc::{RAND_MAX, srand, rand};
use std::cmp::Ordering;

const SEED: u32 = 1;
const MEAN: f64 = 1.0;
const SAMPLES: u32 = 1000;

fn exponential(rand: f64, mean: f64) -> f64 {
    (-1.0*mean)*(rand.ln()) // Log base e.
}

fn inverse_exponential(rand: f64, mean: f64) -> f64 {
    (-1.0*(1.0-rand).ln()) / mean // Log base e.
}

fn float_sort (a: &f64, b: &f64) -> Ordering {
    // Sorting f64s is hard because they can be NaN.
    match a.lt(&b) {
        true => Ordering::Less,
        false => if a.eq(&b) { Ordering::Equal } else { Ordering::Greater }
    }
}

fn main() {
    unsafe { srand(SEED); } // Seed
    let mut values = (0..SAMPLES).map(|_| {
        let num = unsafe { rand() } as f64 / RAND_MAX as f64;
        (exponential(num, MEAN), 0.0f64) // Inverse populated after sort.
    }).collect::<Vec<_>>();

    values.sort_by(|a, b| float_sort(&a.0, &b.0));

    for (index, value) in values.iter_mut().enumerate() {
        let num_for_inverse = (index as f64 - 0.5) / SAMPLES as f64;
        value.1 = inverse_exponential(num_for_inverse, MEAN);
    }

    for &(exponential, inverse_exponential) in values.iter() {
        println!("{}, {}", exponential, inverse_exponential);
    }

    println!("\n\n");
    let mean = values.iter().fold(0f64,|acc, &(expo, _)| acc + expo) / SAMPLES as f64;
    println!("Mean: {}", mean);

    let variance = (values.iter().fold(0f64,|acc, &(expo, _)| {
        acc + expo.powf(2.0)
    }) - (SAMPLES as f64 * mean.powf(2.0))) / (SAMPLES -1) as f64;
    println!("Variance: {}", variance);
}

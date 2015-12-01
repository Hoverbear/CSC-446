extern crate libc;
use libc::{RAND_MAX, srand, rand};
use std::cmp::Ordering;

const SEED: u32 = 1;
const MEAN: f64 = 1.0;
const SAMPLES: u32 = 1000;
const CLASS_INTERVALS: u32 = 30;
const MULTIPLE: f64 = 5.0; // For intervals

fn exponential(rand: f64, mean: f64) -> f64 {
    (-1.0*mean)*(rand.ln()) // Log base e.
}

fn inverse_exponential(rand: f64, mean: f64) -> f64 {
    (-1.0*(1.0-rand).ln()) / mean // Log base e.
}

fn expo_cdf(val: f64, mean: f64) -> f64 {
    
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

    // for &(exponential, inverse_exponential) in values.iter() {
    //     println!("{}, {}", exponential, inverse_exponential);
    // }

    println!("\n\n");
    let mean = values.iter().fold(0f64,|acc, &(expo, _)| acc + expo) / SAMPLES as f64;
    println!("Mean: {}", mean);

    let variance = (values.iter().fold(0f64,|acc, &(expo, _)| {
        acc + expo.powf(2.0)
    }) - (SAMPLES as f64 * mean.powf(2.0))) / (SAMPLES -1) as f64;
    println!("Variance: {}", variance);

    let mut intervals = (0..CLASS_INTERVALS).map(|_| 0).collect::<Vec<_>>();
    for (value, _) in values {
        // Over 6 is for the [30] index.
        let mut index = (value * MULTIPLE) as usize;
        if index >= intervals.len() {
            index = intervals.len() -1;
        }
        // println!("{} {}", index, value);
        intervals[index] += 1;
    }
    println!("Class Interval Buckets: {:#?}", intervals);

    let chi_squared = intervals.iter().enumerate().fold(0.0f64, |mut acc, (index, &value)| {
        let x = (index as f64 * CLASS_INTERVALS as f64) / SAMPLES as f64 + 0.015; // Midpoint
        println!("{}", x);
        let expected_frequency = SAMPLES as f64 * exponential(x, MEAN);
        println!("Expected Frequency: {}", expected_frequency);
        let result = (value as f64 - expected_frequency).powf(2.0) / expected_frequency;
        acc += result;
        acc
    });
    println!("Chi Squared: {}", chi_squared);
}

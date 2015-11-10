extern crate libc;
use libc::{RAND_MAX, srand, rand};

const SEED: u32 = 1;
const MEAN: f64 = 1.0;
const INCREMENT_SIZE: f64 = 0.5;
const SAMPLES: u32 = 1000;
const BINS: usize = 11;

fn exponential(mean: f64) -> f64 {
    let num = unsafe { rand() } as f64 / RAND_MAX as f64;
    (-1.0*mean)*num.ln() // Log base e.
}

fn main() {
    unsafe { srand(SEED); } // Seed
    let values = (0..SAMPLES).map(|_| exponential(MEAN)).collect::<Vec<_>>();
    let mut bins = (0..BINS).map(|_| 0).collect::<Vec<_>>();

    for value in values {
        for (index, bin) in bins.iter_mut().enumerate() {
            if value < ((index+1) as f64) * INCREMENT_SIZE {
                *bin += 1;
                break
            }
        }
        // If we didn't break we should put this in the last bin.
        if value > BINS as f64 * INCREMENT_SIZE {
            let len = bins.len();
            bins[len-1] += 1;
        }
    }
    println!("Generated bins: {:#?}", bins);

    // Normalize
    let normalized = bins.iter().map(|&bin| bin as f64 / SAMPLES as f64).collect::<Vec<_>>();
    println!("Normalized bins: {:#?}", normalized);
}

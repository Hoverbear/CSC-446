extern crate libc;
use libc::{RAND_MAX, srand, rand};

const SEED: u32 = 1;
const MEAN: f64 = 1.0;

fn exponential(mean: f64) -> f64 {
    let num = unsafe { rand() } as f64 / RAND_MAX as f64;
    (-1.0*mean)*num.ln() // Log base e.
}

fn main() {
    unsafe { srand(SEED); } // Seed
    for _ in 0..10 {
        println!("{}", exponential(MEAN));
    }
}

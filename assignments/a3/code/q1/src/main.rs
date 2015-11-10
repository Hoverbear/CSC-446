extern crate rand;
use rand::distributions::{IndependentSample, Range};

const SAMPLES: usize = 1000; // Must be greater than 2.
const NUM_INTERVALS: u64 = 10;
const RANGE_BOTTOM: f64 = 0.0;
const RANGE_TOP: f64 = 1.0;
const TABLE_RESULT: f64 = 16.9; // x_0.05^2 with 9 DOF

fn generate() -> Vec<f64> {
    // Necessary for the first two...
    let range = Range::new(RANGE_BOTTOM, RANGE_TOP);
    let mut rng = rand::thread_rng();
    
    // Initialize the output.
    let mut output = vec![
        range.ind_sample(&mut rng),
        range.ind_sample(&mut rng),
    ];
    
    // Calculate future values.
    for index in 2..SAMPLES {
        let mut next = output[index-1] + output[index-2];
        if next >= 1.0 {
            next = next - 1.0
        }
        output.push(next)
    }
    output
}

fn chi_squared_test(values: &[f64], intervals: u64, expected: f64) {
    let samples = values.len();
    let expected_in_bins = samples as u64 / intervals;
    let interval_size = 1.0 / (intervals as f64);
    
    // Create bins.
    let mut bins = Vec::new();
    for _ in 0..intervals {
        bins.push(0);
    }
    
    // Bin values.
    for value in values {
        let destination_bin = (value / interval_size) as usize;
        bins[destination_bin] += 1;
    }
    println!("Bins: {:?}", bins);
    
    // (O_i - E_i)^2 / E_i
    let sum = bins.iter().fold(0.0f64, |mut acc, bin| {
        let computed = (bin - expected_in_bins as isize).pow(2) as f64 / expected_in_bins as f64;
        acc += computed;
        acc
    });
    
    println!("Chi Squared result: {}, expected: {}", sum, expected);
    
    // Accept or reject based on table.
    if sum > expected {
        println!("Reject");
    } else {
        println!("Accept");
    }
}

fn main() {
    // Generate 'randoms'.
    let randoms = generate();
    chi_squared_test(&randoms, NUM_INTERVALS, TABLE_RESULT);
    
}

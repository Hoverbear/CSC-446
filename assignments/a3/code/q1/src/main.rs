extern crate rand;
use rand::distributions::{IndependentSample, Range};

const SAMPLES: usize = 1000; // Must be greater than 2.
const NUM_INTERVALS: u64 = 10;
const RANGE_BOTTOM: f64 = 0.0;
const RANGE_TOP: f64 = 1.0;
const TABLE_RESULT: f64 = 16.9;
const EXPECTED_IN_BINS: u64 = (SAMPLES as u64 / NUM_INTERVALS);
const INTERVAL_SIZE: f64 = (RANGE_TOP - RANGE_BOTTOM) / (NUM_INTERVALS as f64);

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

fn main() {
    // Generate 'randoms'.
    let randoms = generate();
    
    // Create bins.
    let mut bins = Vec::new();
    for _ in 0..NUM_INTERVALS {
        bins.push(0);
    }
    
    // Bin values.
    for random in randoms {
        let destination_bin = (random / INTERVAL_SIZE) as usize;
        bins[destination_bin] += 1;
    }
    println!("Bins: {:?}", bins);
    
    // (O_i - E_i)^2 / E_i
    let sum = bins.iter().fold(0.0f64, |mut acc, bin| {
        let computed = (bin - EXPECTED_IN_BINS as isize).pow(2) as f64 / EXPECTED_IN_BINS as f64;
        acc += computed;
        acc
    });
    
    println!("Chi Squared result: {}, expected: {}", sum, TABLE_RESULT);
    
    // Accept or reject based on table.
    if sum > TABLE_RESULT {
        println!("Reject");
    } else {
        println!("Accept");
    }
}

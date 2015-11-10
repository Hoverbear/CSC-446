extern crate libc;
use libc::c_uint;
use std::cmp::Ordering;

const SEED: usize = 6;

const KS_SAMPLES: usize = 10;    // CHI_SAMPLES MUST be greater than KS_SAMPLES since
const CHI_SAMPLES: usize = 1000; // it uses the single generated set of randoms.
const KS_TABLE_VALUE: f64 = 0.410; // x_0.05^2 with 9 DOF
const CHI_TABLE_VALUE: f64 = 16.9;
const CHI_INTERVALS: u64 = 10;

fn seed_rand(seed: usize) {
    unsafe { libc::srand(seed as c_uint) }
}

fn rand() -> i32 {
    unsafe { libc::rand() }
}

fn float_sort (a: &f64, b: &f64) -> Ordering {
    // Sorting f64s is hard because they can be NaN.
    match a.lt(&b) {
        true => Ordering::Less,
        false => if a.eq(&b) { Ordering::Equal } else { Ordering::Greater }
    }
}

// Sorts `values` for you as well.
fn ks_test(values: &mut [f64], critical_value: f64) {
    println!("--- Beginning KS Test ---");
    let length = values.len();
    
    // Sort them smallest to largest.
    values.sort_by(float_sort);
    
    // Calculate the values as per slide 33, chapter 8.
    let mut d_plus_set = values.iter().enumerate().map(|(index, &r_i)| {
        (index / length) as f64 - (r_i)
    }).collect::<Vec<_>>();
    d_plus_set.sort_by(float_sort);
    // Get the max.
    let d_plus = d_plus_set.last().unwrap();
    println!("d_plus: {:#?}", d_plus);
    
    // Calculate the values as per slide 33, chapter 8.
    let mut d_minus_set = values.iter().enumerate().map(|(index, &r_i)| {
        (r_i) - ((index as f64 -1.0f64) / length as f64)
    }).collect::<Vec<_>>();
    d_minus_set.sort_by(float_sort);
    // Get the Max
    let d_minus = d_minus_set.last().unwrap();
    println!("d_minus: {:#?}", d_minus);
    
    // Choose the max.
    let d_max = if d_plus >= d_minus { d_plus } else { d_minus };
    println!("d_max: {:#?}", d_max);
    println!("Critical Value: {:#?}", critical_value);
    
    // Decide on reject/accept.
    if *d_max <= critical_value {
        println!("Choice: Accept.");
    } else {
        println!("Choice: Reject");
    }
}

fn chi_squared_test(values: &[f64], intervals: u64, expected: f64) {
    println!("--- Beginning Chi Squared Test ---");
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
            let destination_bin = (*value as f64 / interval_size) as usize;
            bins[destination_bin] += 1;
        }
        println!("Instance bins: {:?}", bins);
        
        // (O_i - E_i)^2 / E_i
        let sum = bins.iter().fold(0.0f64, |mut acc, bin| {
            let computed = (bin - expected_in_bins as isize).pow(2) as f64 / expected_in_bins as f64;
            acc += computed;
            acc
        });
        
        println!("Chi Squared Result: {}", sum);
        println!("Critical Value: {}", expected);
        
        // Accept or reject based on table.
        if sum > expected {
            println!("Choice: Reject.");
        } else {
            println!("Choice: Accept.");
        }
    }

fn main() {
    // Seed and gather a sample of randoms.
    seed_rand(SEED);
    let mut randoms = (0..CHI_SAMPLES)
        .map(|_| (rand() % 100) as f64 / 100.0f64)
        .collect::<Vec<_>>();
    
    ks_test(&mut randoms[0..KS_SAMPLES], KS_TABLE_VALUE);
    chi_squared_test(&mut randoms[0..CHI_SAMPLES], CHI_INTERVALS, CHI_TABLE_VALUE);
}

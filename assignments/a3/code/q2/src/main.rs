extern crate libc;
use libc::c_uint;
use std::cmp::Ordering;

const SEED: usize = 3;

const KS_SAMPLES: usize = 10;
const KS_TABLE_VALUE: f64 = 0.410;

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

fn ks_test() {
    println!("--- Beginning KS Test ---");
    
    // Seed and gather a sample of randoms.
    seed_rand(SEED);
    let mut randoms = (0..KS_SAMPLES)
        .map(|_| (rand() % 100) as f64 / 100.0f64)
        .collect::<Vec<_>>();
        
    // Sort them smallest to largest.
    randoms.sort_by(float_sort);
    println!("rands {:?}", randoms);
    
    // Calculate the values as per slide 33, chapter 8.
    let mut d_plus_set = randoms.iter().enumerate().map(|(index, &r_i)| {
        (index / KS_SAMPLES) as f64 - (r_i)
    }).collect::<Vec<_>>();
    d_plus_set.sort_by(float_sort);
    // Get the max.
    let d_plus = d_plus_set.last().unwrap();
    println!("d_plus: {:#?}", d_plus);
    
    // Calculate the values as per slide 33, chapter 8.
    let mut d_minus_set = randoms.iter().enumerate().map(|(index, &r_i)| {
        (r_i) - ((index as f64 -1.0f64) / KS_SAMPLES as f64)
    }).collect::<Vec<_>>();
    d_minus_set.sort_by(float_sort);
    // Get the Max
    let d_minus = d_minus_set.last().unwrap();
    println!("d_minus: {:#?}", d_minus);
    
    // Choose the max.
    let d_max = if d_plus >= d_minus { d_plus } else { d_minus };
    println!("d_max: {:#?}", d_max);
    println!("Critical Value: {:#?}", KS_TABLE_VALUE);
    
    // Decide on reject/accept.
    if *d_max <= KS_TABLE_VALUE {
        println!("Choice: Accept.");
    } else {
        println!("Choice: Reject");
    }
}

fn main() {
    ks_test();
    
}

extern crate rand;
use rand::distributions::{IndependentSample, Range};

fn generate_1k() -> Vec<f64> {
    // Necessary for the first two...
    let range = Range::new(0.0f64, 1.0);
    let mut rng = rand::thread_rng();
    // Initialize the output.
    let mut output = vec![
        range.ind_sample(&mut rng),
        range.ind_sample(&mut rng),
    ];
    for index in 2..1000 {
        let mut next = output[index-1] + output[index-2];
        if next >= 1.0 {
            next = next - 1.0
        }
        output.push(next)
    }
    output
}

fn main() {
    let result = generate_1k();
    
    println!("{:?}", result);
}

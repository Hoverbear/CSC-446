extern crate rand;
use rand::distributions::{IndependentSample, Range};

const P: f64 = 0.4;
const SAMPLE: usize = 100_000_000;

#[derive(Eq, PartialEq)]
enum Coin {
    Heads,
    Tails
}

struct CoinTosser {
    rng: rand::ThreadRng,
    range: Range<f64>,
}
impl Iterator for CoinTosser {
    type Item = Coin;
    fn next(&mut self) -> Option<Coin> {
        let val = self.range.ind_sample(&mut self.rng);
        if val < P {
            Some(Coin::Heads)
        } else {
            Some(Coin::Tails)
        }
    }
}

fn main() {
    let tosser = CoinTosser {
        rng: rand::thread_rng(),
        range: Range::new(0.0f64, 1.0),
    };
    let p_hat = tosser.take(SAMPLE).fold(0.0f64, |acc, val| {
        if val == Coin::Heads { acc+1.0 }
        else { acc }
    }) / SAMPLE as f64;

    println!("p hat: {}", p_hat);
    println!("Normalized estimation error: {}", (P - p_hat).abs() / P);

}

fn linear_congruential_method(last: u64, multiplier: u64, increment: u64, modulus: u64) -> u64 {
    ((multiplier * last) + increment) % (modulus)
}

fn find_lcm_cycle(x_0: u64, multipler: u64, increment: u64, modulus: u64) -> Vec<u64> {
    let mut x_list = vec![x_0];
    let mut index = 1; // Prepopulated with x_0;
    loop {
        let next = linear_congruential_method(x_list[index-1], multipler, increment, modulus);

        if x_list.iter().find(|&&v| v == next).is_some() {
            break; // Done, found a cycle.
        }

        x_list.push(next);
        index += 1;
    }
    x_list
}

fn main() {
    println!("a=11, m=16, x_0=7 -> {:?}", find_lcm_cycle(7, 11, 0, 16));
    println!("a=11, m=16, x_0=8 -> {:?}", find_lcm_cycle(8, 11, 0, 16));
    println!("a=7, m=16, x_0=7 -> {:?}", find_lcm_cycle(7, 7, 0, 16));
    println!("a=7, m=16, x_0=8 -> {:?}", find_lcm_cycle(8, 7, 0, 16));
}

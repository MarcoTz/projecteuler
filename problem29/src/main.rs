use num_bigint::BigUint;
use std::collections::HashSet;

const MIN_A: usize = 2;
const MAX_A: usize = 100;
const MIN_B: usize = 2;
const MAX_B: usize = 100;

fn main() {
    let mut powers = HashSet::new();
    for a in MIN_A..=MAX_A {
        for b in MIN_B..=MAX_B {
            let a_big: BigUint = a.into();
            let next_pow = a_big.pow(b as u32);
            powers.insert(next_pow);
        }
    }
    let mut power_vec: Vec<&BigUint> = powers.iter().collect();
    power_vec.sort();
    println!("number: {}", power_vec.len());
}

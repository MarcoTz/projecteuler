use std::{collections::HashMap, fmt, process::exit};
pub mod num;
use num::Num;

const NUM_DIGITS: usize = 1000;

type FibCache = HashMap<u128, Num>;

fn fib(n: u128, cache: &mut FibCache) -> Num {
    if let Some(fib) = cache.get(&n) {
        return fib.clone();
    }
    if n == 1 || n == 2 {
        return 1.into();
    }

    let res = fib(n - 1, cache) + fib(n - 2, cache);
    cache.insert(n, res.clone());
    res
}

fn main() {
    let mut num: u128 = 1;
    let mut cache = HashMap::default();
    loop {
        let fib = fib(num, &mut cache);
        if fib.num_digits() >= NUM_DIGITS {
            println!("{num}th Fibonacci number has over {NUM_DIGITS} digits:\n{fib}");
            exit(0)
        }
        num += 1;
    }
}

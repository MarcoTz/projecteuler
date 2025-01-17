use std::fmt;

mod circular_buffer;
use circular_buffer::CircularBuffer;

#[derive(Default)]
struct NumFactors {
    num: u64,
    num_factors: u64,
}

fn distinct_prime_factors(n: u64) -> u64 {
    let upper_bound = (n as f64).sqrt().ceil() as u64;
    let mut possible: Vec<u64> = (2..=upper_bound).collect();

    let mut remaining = n;
    let mut num_factors = 0;
    while remaining != 1 {
        if possible.is_empty() {
            return num_factors + 1;
        }
        let next_prime = possible.first().unwrap();
        if remaining % next_prime == 0 {
            num_factors += 1;
        }
        while remaining % next_prime == 0 {
            remaining /= next_prime;
        }
        possible = possible
            .iter()
            .filter_map(|num| (num % next_prime != 0).then_some(*num))
            .collect();
    }
    num_factors
}

fn main() {
    let mut buf: CircularBuffer<NumFactors, 4> = CircularBuffer::new();

    let mut num = 2;
    loop {
        let num_factors = NumFactors {
            num,
            num_factors: distinct_prime_factors(num),
        };
        buf.push(num_factors);
        if buf.all(|facs| facs.num_factors == 4) {
            println!("{buf}");
            break;
        }
        num += 1;
    }
}

impl fmt::Display for NumFactors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {} factors", self.num, self.num_factors)
    }
}

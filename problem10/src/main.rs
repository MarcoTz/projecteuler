const MAX_NUM: u64 = 2000000;

struct PrimeSieve {
    primes: Vec<u64>,
    remaining: Vec<u64>,
}

impl PrimeSieve {
    fn new(max: u64) -> PrimeSieve {
        PrimeSieve {
            primes: vec![],
            remaining: (2..=max).collect(),
        }
    }

    fn search(&mut self) {
        while !self.remaining.is_empty() {
            let next_prime = self.remaining.remove(0);
            self.primes.push(next_prime);
            self.remaining = self
                .remaining
                .iter()
                .filter_map(|num| (num % next_prime != 0).then_some(*num))
                .collect();
            println!("remaining: {}", self.remaining.len());
        }
    }
}

fn main() {
    let mut sieve = PrimeSieve::new(MAX_NUM);
    sieve.search();
    println!("{}", sieve.primes.iter().sum::<u64>());
}

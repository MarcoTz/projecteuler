const MAX_CHECK: u64 = 1000000;

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
            let next_prime = *self.remaining.first().unwrap();
            self.primes.push(next_prime);
            self.remaining = self
                .remaining
                .iter()
                .filter_map(|num| (num % next_prime != 0).then_some(*num))
                .collect();
        }
    }
}
fn main() {
    let mut sieve = PrimeSieve::new(MAX_CHECK);
    sieve.search();
    let primes = sieve.primes;
    println!("got primes up to {MAX_CHECK}");

    let max_prime = primes.last().unwrap();
    let mut sums = vec![];
    for window_size in 2..=primes.len() {
        for n_primes in primes.windows(window_size) {
            let sum: u64 = n_primes.iter().sum();
            if sum > *max_prime {
                break;
            }
            if primes.contains(&sum) {
                sums.push((n_primes, sum));
            }
        }
        println!("finished calculating sums of {window_size} primes")
    }
    let max_sum = sums.last().unwrap();
    println!(
        "{} = {}",
        max_sum.1,
        max_sum
            .0
            .iter()
            .map(|prime| prime.to_string())
            .collect::<Vec<String>>()
            .join("+"),
    );
}

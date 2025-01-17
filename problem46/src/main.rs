struct PrimeSieve {
    max: u64,
    primes: Vec<u64>,
    remaining: Vec<u64>,
}

impl PrimeSieve {
    fn new(max: u64) -> PrimeSieve {
        PrimeSieve {
            max,
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

    fn extend(&mut self, new_max: u64) {
        let mut additional_remaining: Vec<u64> = (self.max..=new_max).collect();
        for prime in self.primes.iter() {
            additional_remaining = additional_remaining
                .iter()
                .filter_map(|num| (num % prime != 0).then_some(*num))
                .collect();
        }
        self.max = new_max;
        self.remaining.extend(additional_remaining);
    }
}

fn goldbach_sum(num: u64, primes: &[u64]) -> Option<(u64, u64)> {
    for prime in primes.iter() {
        if *prime > num || num % 2 == 0 {
            return None;
        }
        let diff = (num - prime) / 2;
        let sqrt = (diff as f64).sqrt();
        if sqrt.round() == sqrt {
            return Some((*prime, sqrt as u64));
        }
    }
    None
}

fn main() {
    let mut next_composite = 3;
    let mut searcher = PrimeSieve::new(100);
    searcher.search();

    loop {
        if searcher.max < next_composite {
            searcher.extend(next_composite * 10);
            searcher.search();
        }
        if let Some((prime, square)) = goldbach_sum(next_composite, &searcher.primes) {
            println!("{}={}+2*{}^2", next_composite, prime, square);
        } else {
            break;
        }
        next_composite += 2;
    }
    println!("{}", next_composite);
}

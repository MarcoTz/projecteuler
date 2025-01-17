use std::fmt;

pub struct PrimeSieve {
    max_prime: u64,
    primes: Vec<u64>,
    remaining: Vec<u64>,
}

pub struct PrimePower {
    prime: u64,
    power: u32,
}

#[derive(Default)]
pub struct Factorization {
    num: u64,
    factors: Vec<PrimePower>,
}

impl Factorization {
    pub fn distinct_factors(&self, other: &Factorization) -> bool {
        let self_primes = self.factors.iter().map(|pow| pow.prime);
        let other_primes: Vec<u64> = other.factors.iter().map(|pow| pow.prime).collect();
        for prime in self_primes {
            if other_primes.contains(&prime) {
                return false;
            }
        }
        true
    }
}

impl PrimeSieve {
    pub fn new(max: u64) -> PrimeSieve {
        PrimeSieve {
            max_prime: 2,
            primes: vec![2],
            remaining: (3..=max).collect(),
        }
    }

    pub fn search(&mut self) {
        while !self.remaining.is_empty() {
            let next_prime = *self.remaining.first().unwrap();
            self.primes.push(next_prime);
            self.remaining = self
                .remaining
                .iter()
                .filter_map(|num| (num % next_prime != 0).then_some(*num))
                .collect();
            self.max_prime = next_prime;
        }
    }

    fn extend(&mut self, new_max: u64) {
        let mut additional_remaining: Vec<u64> = (self.max_prime..=new_max).collect();
        for prime in self.primes.iter() {
            additional_remaining = additional_remaining
                .iter()
                .filter_map(|num| (num % prime != 0).then_some(*num))
                .collect();
        }
        self.remaining.extend(additional_remaining);
    }

    pub fn factor(&mut self, num: u64) -> Factorization {
        while self.max_prime < num {
            self.extend(num * 10);
            self.search();
        }
        let mut remaining = num;
        let mut factors = vec![];
        for prime in self.primes.iter() {
            let mut power = 0;
            while remaining % prime == 0 {
                power += 1;
                remaining = remaining / prime;
            }
            if power > 0 {
                factors.push(PrimePower {
                    prime: *prime,
                    power,
                });
            }
        }
        Factorization { num, factors }
    }
}

impl fmt::Display for PrimePower {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.power > 1 {
            write!(f, "{}^{}", self.prime, self.power)
        } else {
            self.prime.fmt(f)
        }
    }
}

impl fmt::Display for Factorization {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} = {}",
            self.num,
            self.factors
                .iter()
                .map(|fac| fac.to_string())
                .collect::<Vec<String>>()
                .join("+")
        )
    }
}

pub struct PrimeSieve {
    pub primes: Vec<usize>,
    max_checked: usize,
}

impl PrimeSieve {
    pub fn is_prime(&mut self, num: usize) -> bool {
        if self.max_checked >= num {
            return self.primes.contains(&num);
        }

        for i in self.max_checked..=num {
            let mut is_prime = true;
            let primes = self.primes.clone();
            for prime in primes.iter() {
                if i % prime == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                self.primes.push(i);
            }
        }
        self.max_checked = num;
        self.primes.contains(&num)
    }
}

impl Default for PrimeSieve {
    fn default() -> PrimeSieve {
        PrimeSieve {
            primes: vec![2],
            max_checked: 2,
        }
    }
}

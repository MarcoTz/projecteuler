use std::collections::HashSet;

const MAX_NUM: usize = 1000000;

struct PrimeSearcher {
    max_checked: usize,
    primes: HashSet<usize>,
}

impl Default for PrimeSearcher {
    fn default() -> PrimeSearcher {
        PrimeSearcher {
            max_checked: 1,
            primes: HashSet::new(),
        }
    }
}

impl PrimeSearcher {
    fn check(&mut self, num: usize) -> bool {
        if self.max_checked >= num {
            return self.primes.contains(&num);
        }

        for i in (self.max_checked + 1)..=num {
            let primes = self.primes.clone();
            let mut is_prime = true;
            for prime in primes {
                if i % prime == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                self.primes.insert(i);
            }
        }

        self.max_checked = num;
        self.primes.contains(&num)
    }
}

fn digits(num: usize) -> Vec<usize> {
    let mut num = num;
    let mut digits = vec![];
    while num >= 10 {
        digits.push(num % 10);
        num = num / 10;
    }
    digits.push(num);
    digits.reverse();
    digits
}
fn digits_to_num(digits: &Vec<usize>) -> usize {
    let mut num = 0;
    for (ind, digit) in digits.into_iter().rev().enumerate() {
        num += 10_usize.pow(ind as u32) * digit;
    }
    num
}

fn circular(prime: usize, primes: &HashSet<usize>) -> bool {
    let digits = digits(prime);
    let mut rotated = digits.clone();
    rotated.rotate_left(1);
    while rotated != digits {
        if !primes.contains(&digits_to_num(&rotated)) {
            return false;
        }
        rotated.rotate_left(1);
    }
    true
}

fn main() {
    let mut searcher = PrimeSearcher::default();
    for i in 1..=MAX_NUM {
        searcher.check(i);
        if i % 10000 == 0 {
            println!("checked {i}");
        }
    }
    println!(
        "found all primes below {MAX_NUM} ({})",
        searcher.primes.len()
    );
    let primes_cloned = searcher.primes.clone();
    let mut circ_primes = vec![];
    for prime in primes_cloned {
        if circular(prime, &searcher.primes) {
            circ_primes.push(prime);
        }
    }
    println!("circular numbers : {circ_primes:?}");
    println!("number of circular: {}", circ_primes.len());
}

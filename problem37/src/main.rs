const MAX_NUM: usize = 1000000;

struct PrimeSearcher {
    max_checked: usize,
    remaining: Vec<usize>,
    primes: Vec<usize>,
}

impl PrimeSearcher {
    fn check(&mut self, max: usize) {
        for i in self.max_checked..=max {
            if !self.remaining.contains(&i) {
                continue;
            }

            self.primes.push(i);
            self.remaining = self
                .remaining
                .iter()
                .filter_map(|num| if num % i == 0 { None } else { Some(*num) })
                .collect();
            println!("found prime {i}, remaining {}", self.remaining.len());
        }
        self.max_checked = max;
    }

    fn right_truncatable(&mut self, num: usize) -> bool {
        self.check(num);
        let mut left = num / 10;
        while left != 0 {
            if !self.primes.contains(&left) {
                return false;
            }
            left = left / 10
        }
        true
    }

    fn left_truncatable(&mut self, num: usize) -> bool {
        self.check(num);
        let mut max_power = 10;
        while max_power * 10 < num {
            max_power = max_power * 10;
        }
        if num == 29 {
            println!("max power: {max_power}");
        }
        let mut right = num % max_power;
        while right != 0 {
            if !self.primes.contains(&right) {
                return false;
            }
            max_power = max_power / 10;
            right = right % max_power;
        }
        true
    }
}

impl Default for PrimeSearcher {
    fn default() -> PrimeSearcher {
        PrimeSearcher {
            max_checked: 1,
            remaining: (2..=MAX_NUM).collect(),
            primes: vec![],
        }
    }
}
fn main() {
    println!("{}", 3_usize / 10);
    let mut searcher = PrimeSearcher::default();
    searcher.check(MAX_NUM);
    println!("finished checking primes, {}", searcher.primes.len());
    let mut truncatable: Vec<usize> = searcher
        .primes
        .clone()
        .iter()
        .filter(|num| **num > 10_usize)
        .filter_map(|num| searcher.left_truncatable(*num).then_some(*num))
        .collect();
    println!("checked left truncatable {}", truncatable.len());
    truncatable = truncatable
        .iter()
        .filter_map(|num| searcher.right_truncatable(*num).then_some(*num))
        .collect();
    println!("checked right truncatable {}", truncatable.len());
    println!("sum: {}", truncatable.iter().sum::<usize>());
}

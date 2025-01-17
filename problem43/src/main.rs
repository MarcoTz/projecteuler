use std::iter;

const MAX_DIGIT: usize = 9;

struct PrimeSieve {
    remaining: Vec<u64>,
    primes: Vec<u64>,
}

impl PrimeSieve {
    fn new(max: usize) -> PrimeSieve {
        PrimeSieve {
            remaining: (2..=max).map(|num| num as u64).collect(),
            primes: vec![],
        }
    }

    fn search(&mut self) {
        while !self.remaining.is_empty() {
            let next_prime = self.remaining.first().unwrap();
            self.primes.push(*next_prime);
            self.remaining = self
                .remaining
                .iter()
                .filter_map(|num| (num % next_prime != 0).then_some(*num))
                .collect()
        }
    }
}

struct HeapAlgo {
    nums: Vec<u64>,
    loop_count: Vec<usize>,
    loop_num: usize,
}

impl HeapAlgo {
    fn new(max_num: usize) -> HeapAlgo {
        HeapAlgo {
            nums: (0..=max_num).map(|num| num as u64).collect(),
            loop_count: iter::repeat(0).take(max_num + 1).collect(),
            loop_num: 0,
        }
    }
}

impl Iterator for HeapAlgo {
    type Item = Vec<u64>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.loop_num == 0 {
            self.loop_num = 1;
            return Some(self.nums.clone());
        }

        if self.loop_num >= self.nums.len() {
            return None;
        }

        if self.loop_count[self.loop_num] < self.loop_num {
            if self.loop_num % 2 == 0 {
                self.nums.swap(0, self.loop_num);
            } else {
                self.nums
                    .swap(self.loop_count[self.loop_num], self.loop_num);
            }
            self.loop_count[self.loop_num] += 1;
            self.loop_num = 1;
            Some(self.nums.clone())
        } else {
            self.loop_count[self.loop_num] = 0;
            self.loop_num += 1;
            self.next()
        }
    }
}

fn vec2num(vec: Vec<u64>) -> u64 {
    let num_digits = vec.len();
    vec.iter()
        .enumerate()
        .map(|(ind, dig)| 10_u64.pow((num_digits - 1 - ind) as u32) * (*dig as u64))
        .sum()
}

fn substrs(vec: Vec<u64>) -> Vec<[u64; 3]> {
    let mut substrs = vec![];
    for i in 1..vec.len() - 2 {
        substrs.push([vec[i], vec[i + 1], vec[i + 2]]);
    }
    substrs
}

fn main() {
    let mut sieve = PrimeSieve::new(MAX_DIGIT * 10);
    sieve.search();

    let mut gen = HeapAlgo::new(MAX_DIGIT);
    let mut pandigitals = vec![];
    'outer: while let Some(perm) = gen.next() {
        if perm[0] == 0 {
            continue;
        }
        let substrs = substrs(perm.clone());
        for (ind, substr) in substrs.iter().enumerate() {
            let nth_prime = sieve.primes[ind];
            let num = vec2num(substr.to_vec());
            if num % nth_prime != 0 {
                continue 'outer;
            }
        }
        pandigitals.push(vec2num(perm));
    }
    println!("{}", pandigitals.iter().sum::<u64>());
}

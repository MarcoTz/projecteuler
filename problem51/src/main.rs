use std::collections::HashSet;
struct DigitGen {
    last_num: Vec<u8>,
}

impl DigitGen {
    fn new() -> DigitGen {
        DigitGen { last_num: vec![0] }
    }
}

impl Iterator for DigitGen {
    type Item = Vec<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        let last_ind = self.last_num.len() - 1;
        self.last_num[last_ind] += 1;
        self.last_num[last_ind] = self.last_num[last_ind] % 10;
        if self.last_num[last_ind] != 0 {
            return Some(self.last_num.clone());
        }
        let mut next_index = last_ind;
        while self.last_num[next_index] == 0 {
            if next_index == 0 {
                break;
            }

            next_index -= 1;
            self.last_num[next_index] += 1;
            self.last_num[next_index] = self.last_num[next_index] % 10;
        }

        if self.last_num[0] == 0 {
            self.last_num.insert(0, 1);
        }

        Some(self.last_num.clone())
    }
}

fn is_prime(n: u64) -> bool {
    if n == 0 {
        return false;
    }
    let upper_bound = (n as f64).sqrt().ceil() as u64;
    for i in 2..=upper_bound {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn digits2num(digits: Vec<u8>) -> u64 {
    let mut num = 0;
    for (ind, dig) in digits.into_iter().rev().enumerate() {
        num += 10_u64.pow(ind as u32) * (dig as u64);
    }
    num
}

fn generate_indices(num_replacements: u8, num_digits: u8) -> HashSet<Vec<usize>> {
    if num_replacements == 0 || num_digits == 0 || num_replacements > num_digits {
        return HashSet::new();
    }
    if num_replacements == 1 {
        return (0..(num_digits as usize)).map(|num| vec![num]).collect();
    }
    if num_digits == 1 {
        return HashSet::from([vec![0]]);
    }

    let mut without_last = generate_indices(num_replacements, num_digits - 1);
    let with_last = generate_indices(num_replacements - 1, num_digits);
    for mut repl in with_last {
        if repl.contains(&(num_digits as usize - 1)) {
            continue;
        }
        repl.push(num_digits as usize - 1);
        without_last.insert(repl);
    }
    without_last
}

fn main() {
    let mut gen = DigitGen::new();
    'main: loop {
        let next_num = gen.next().unwrap();
        let num_num = digits2num(next_num.clone());
        if !is_prime(num_num) {
            continue;
        }

        let num_digits = next_num.len();
        for i in 1..=num_digits {
            let replacements = generate_indices(i as u8, num_digits as u8);
            for replacement in replacements.iter() {
                let mut nums = vec![];
                for repl_target in 0..=9 {
                    let mut new_num = next_num.clone();
                    for ind in replacement {
                        new_num[*ind] = repl_target;
                    }
                    if new_num[0] == 0 {
                        continue;
                    }
                    nums.push(digits2num(new_num));
                }
                nums = nums.into_iter().filter(|num| is_prime(*num)).collect();
                if nums.len() == 8 {
                    println!("found family for {num_num}; {nums:?}");
                    break 'main;
                }
            }
        }
    }
}

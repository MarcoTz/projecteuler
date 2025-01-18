use std::collections::HashSet;

fn is_prime(num: u64) -> bool {
    let upper_bound = (num as f64).sqrt().ceil() as u64;
    for i in 2..=upper_bound {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn permutations<T: Copy>(vec: Vec<T>) -> Vec<Vec<T>> {
    if vec.is_empty() {
        return vec![];
    }

    if vec.len() == 1 {
        return vec![vec];
    }

    let mut vec = vec;
    let first = vec.remove(0);
    let previous = permutations(vec);
    let mut permutations = vec![];
    for mut perm in previous {
        for i in 0..perm.len() {
            let mut next = perm.clone();
            next.insert(i, first.clone());
            permutations.push(next);
        }
        perm.push(first);
        permutations.push(perm);
    }

    permutations
}

fn num2digits(num: u64) -> Vec<u8> {
    let mut num = num;
    let mut digits = vec![];
    while num >= 10 {
        digits.push((num % 10) as u8);
        num = num / 10;
    }
    digits.push(num as u8);
    digits.reverse();
    digits
}

fn digits2num(digits: Vec<u8>) -> u64 {
    let mut num = 0;
    for (power, dig) in digits.iter().rev().enumerate() {
        num += 10_u64.pow(power as u32) * (*dig as u64);
    }
    num
}

fn digit_permutations(num: u64) -> Vec<u64> {
    let digits = num2digits(num);
    let perms = permutations(digits);
    let num_set: HashSet<u64> = perms.into_iter().map(|digits| digits2num(digits)).collect();
    let mut nums: Vec<u64> = num_set.into_iter().collect();
    nums.sort();
    nums
}

fn check_sequence(nums: Vec<u64>, primes: &[u64]) -> Option<(u64, u64, u64)> {
    //2969 6299 9629
    for first in nums.iter() {
        if !primes.contains(first) {
            return None;
        }

        for second in nums.iter() {
            if !primes.contains(&second) {
                continue;
            }
            if first == second {
                continue;
            }

            let diff = first.abs_diff(*second);
            let candidate_second = second + diff;
            let candidate_first = first + diff;
            if candidate_first != *first
                && candidate_first != *second
                && nums.contains(&candidate_first)
                && primes.contains(&candidate_first)
            {
                return Some((*first, *second, candidate_first));
            }
            if candidate_second != *first
                && candidate_second != *second
                && nums.contains(&candidate_second)
                && primes.contains(&candidate_second)
            {
                return Some((*first, *second, candidate_second));
            }
        }
    }
    todo!()
}

fn main() {
    let mut primes = vec![];
    for i in 1000..9999 {
        if is_prime(i) {
            primes.push(i);
        }
    }
    for prime in primes.iter() {
        let permutations = digit_permutations(*prime);
        if let Some(seq) = check_sequence(permutations.clone(), &primes) {
            println!("{:?}", seq);
            println!("{}{}{}", seq.0, seq.1, seq.2);
            break;
        }
    }
}

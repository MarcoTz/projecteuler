use rand::{distributions::Uniform, rngs::ThreadRng, thread_rng, Rng};

fn mod_power(num: u64, power: u64, modulo: u64) -> u64 {
    let mut power = power - 1;
    let mut res = num;
    while power > 0 {
        res *= num;
        if res > modulo {
            res = res % modulo
        }
        power -= 1;
    }
    res % modulo
}

fn fermat_pseudoprime(num: u64, tries: u64, gen: &mut ThreadRng) -> bool {
    let dist = Uniform::new_inclusive(2, num - 2);
    for _ in 0..tries {
        let witness = gen.sample(dist);
        if mod_power(witness, num - 1, num) != 1 {
            return false;
        }
    }
    true
}

fn is_prime(num: u64, gen: &mut ThreadRng) -> bool {
    let num_tries = (0.25_f64).powi(num as i32).ceil() as u64;
    if !fermat_pseudoprime(num, num_tries, gen) {
        return false;
    }

    let max = (num as f64).sqrt().ceil() as u64;
    for i in 2..=max {
        if num % i == 0 {
            return false;
        }
    }

    true
}

fn digits(num: u64) -> Vec<u8> {
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

fn pandigital(num: u64) -> bool {
    let digits = digits(num);
    let max_digit = digits.len();
    if max_digit > 9 {
        return false;
    }

    for i in 1..=max_digit {
        if !digits.contains(&(i as u8)) {
            return false;
        }
    }

    true
}
fn main() {
    let mut gen = thread_rng();

    for i in 1..9 {
        let mut highest_j = 0;
        for j in 10_usize.pow(i)..10_usize.pow(i + 1) {
            if !pandigital(j as u64) {
                continue;
            }

            if !is_prime(j as u64, &mut gen) {
                continue;
            }
            highest_j = highest_j.max(j);
        }
        if highest_j != 0 {
            println!(
                "biggest pandigital prime with {} digits: {highest_j}",
                i + 1
            );
        } else {
            println!("no pandigital primes with {} digits", i + 1);
        }
    }
}

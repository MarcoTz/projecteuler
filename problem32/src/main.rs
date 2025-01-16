use std::collections::HashSet;

fn factors(num: u64) -> Vec<(u64, u64)> {
    let mut factors = vec![];
    for i in 1..=(num as f64).sqrt().ceil() as u64 {
        if num % i == 0 {
            factors.push((i, num / i))
        }
    }
    factors
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

fn pandigital(a: u64, b: u64, c: u64) -> bool {
    let mut digs = digits(a);
    digs.extend(digits(b));
    digs.extend(digits(c));
    if digs.len() != 9 {
        return false;
    }
    digs.sort();
    digs == (1..=9).collect::<Vec<u8>>()
}

fn main() {
    let mut pandigitals = vec![];
    for c in 1..=10000 {
        let factors = factors(c);
        for (a, b) in factors {
            if pandigital(a, b, c) {
                pandigitals.push((a, b, c));
            }
        }
    }
    let cs: HashSet<u64> = pandigitals.iter().map(|(_, _, c)| *c).collect();
    println!("{}", cs.iter().sum::<u64>());
}

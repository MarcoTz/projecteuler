use std::collections::HashSet;

fn sieve_sum(end: i64) -> i64 {
    let mut candidates: HashSet<i64> = (2..end).collect();
    let mut primes = vec![];
    let mut sum = 0;

    while let Some(i) = candidates.iter().min().cloned() {
        primes.push(i);
        sum += i;
        for n in 1..((end / i) + 1) {
            candidates.remove(&(n * i));
        }
        println!("Finished {i}/{end}");
    }

    return sum;
}

fn main() {
    println!("{:?}", sieve_sum(2000000))
}

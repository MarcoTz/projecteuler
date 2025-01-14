const MAX_A: i64 = 999;
const MAX_B: i64 = 1000;

mod quadratic;
mod sieve;
use quadratic::Quadratic;
use sieve::PrimeSieve;

fn main() {
    let mut sieve = PrimeSieve::default();
    let mut max_primes = 0;
    let mut best_quad = Quadratic { a: 0, b: 0 };
    for a in -MAX_A..=MAX_A {
        for b in -MAX_B..=MAX_B {
            let quad = Quadratic { a, b };
            let primes = quad.consequtive_primes(&mut sieve);
            let num_primes = primes.len();
            if num_primes > max_primes {
                println!("found new best {quad}, with {num_primes} primes");
                max_primes = num_primes;
                best_quad = quad;
            }
        }
    }
    println!(
        "Best quadratic: {best_quad}, number of primes: {max_primes}, product:{}",
        best_quad.a * best_quad.b
    );
}

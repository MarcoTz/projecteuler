use super::sieve::PrimeSieve;
use std::fmt;

pub struct Quadratic {
    pub a: i64,
    pub b: i64,
}

impl Quadratic {
    fn eval(&self, n: usize) -> i64 {
        let n = n as i64;
        n * n + self.a * n + self.b
    }

    pub fn consequtive_primes(&self, sieve: &mut PrimeSieve) -> Vec<usize> {
        let mut n = 0;
        let mut primes = vec![];
        loop {
            let res = self.eval(n).abs() as usize;
            if !sieve.is_prime(res) {
                break;
            }
            primes.push(res);
            n += 1;
        }
        primes
    }
}

impl fmt::Display for Quadratic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a_str = if self.a < 0 {
            format!("{}", self.a)
        } else {
            format!("+{}", self.a)
        };
        let b_str = if self.b < 0 {
            format!("{}", self.b)
        } else {
            format!("+{}", self.b)
        };
        write!(f, "n^2 {}n {}", a_str, b_str)
    }
}

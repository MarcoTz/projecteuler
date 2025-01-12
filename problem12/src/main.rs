struct TriangleGenerator {
    num: usize,
    ind: usize,
}

const MAX_FACTORS: usize = 500;

impl Default for TriangleGenerator {
    fn default() -> TriangleGenerator {
        TriangleGenerator { num: 1, ind: 1 }
    }
}

impl std::iter::Iterator for TriangleGenerator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.ind += 1;
        self.num += self.ind;
        Some(self.num)
    }
}

fn factors(num: usize) -> Vec<usize> {
    let mut factors = vec![1];
    for i in 2..=(num as f64).sqrt().round() as usize {
        if num % i == 0 {
            factors.push(i);
            factors.push(num / i);
        }
    }
    factors
}

fn main() {
    let mut gen = TriangleGenerator::default();
    loop {
        let next = gen.next().unwrap();
        let num_factors = factors(next).len();
        if num_factors >= MAX_FACTORS {
            println!(
                "{}th triangle number:{}, factors: {}",
                gen.ind, gen.num, num_factors
            );
            break;
        }
    }
}

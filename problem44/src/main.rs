struct PentagonGen {
    next_n: u64,
    generated: Vec<u64>,
}

impl PentagonGen {
    fn is_pentagon(num: u64) -> bool {
        let y = ((num * 24 + 1) as f64).sqrt();
        y.round() == y && (y as u64) % 6 == 5
    }

    fn check_sums_diffs(&mut self, num: u64, min_diff: u64) -> u64 {
        let mut min_diff = min_diff;
        for prev in self.generated.iter().rev() {
            let diff = num - prev;
            if diff > min_diff {
                break;
            }
            if Self::is_pentagon(diff) && Self::is_pentagon(prev + num) {
                min_diff = min_diff.min(diff);
            }
        }
        min_diff
    }
}

impl Default for PentagonGen {
    fn default() -> PentagonGen {
        PentagonGen {
            next_n: 1,
            generated: vec![],
        }
    }
}

impl Iterator for PentagonGen {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.next_n * (3 * self.next_n - 1) / 2;
        self.next_n += 1;
        self.generated.push(item);
        Some(item)
    }
}

fn main() {
    let mut gen = PentagonGen::default();

    let mut min_diff = u64::MAX;
    loop {
        let diff = 2 * gen.next_n + 1;
        if diff > min_diff {
            break;
        }
        let next = gen.next().unwrap();
        min_diff = gen.check_sums_diffs(next, min_diff);
    }
    println!("{}", min_diff);
}

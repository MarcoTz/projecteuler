use std::collections::HashSet;

const MAX_IMPOSSIBLE: usize = 28123;

#[derive(Debug)]
enum Perfect {
    Perfect,
    Deficient,
    Abundant,
}

impl From<usize> for Perfect {
    fn from(num: usize) -> Perfect {
        let divs = divisors(num);
        let sum: usize = divs.iter().sum();
        if sum == num {
            Perfect::Perfect
        } else if sum < num {
            Perfect::Deficient
        } else {
            Perfect::Abundant
        }
    }
}

#[derive(Debug, Default)]
struct PerfectNums {
    perfect: Vec<usize>,
    abundant: Vec<usize>,
    deficient: Vec<usize>,
}

impl PerfectNums {
    fn calc_nums(max: usize) -> PerfectNums {
        let mut nums = PerfectNums::default();
        for i in 1..=max {
            nums.add_num(i);
        }
        nums
    }

    fn add_num(&mut self, num: usize) {
        match num.into() {
            Perfect::Perfect => self.perfect.push(num),
            Perfect::Abundant => self.abundant.push(num),
            Perfect::Deficient => self.deficient.push(num),
        }
    }

    fn abundant_sum(&self, num: usize) -> Option<(usize, usize)> {
        let abundant_leq: Vec<&usize> = self.abundant.iter().filter(|i| **i <= num).collect();
        for i in abundant_leq.iter() {
            let diff = num - **i;
            if abundant_leq.contains(&&&diff) {
                return Some((**i, diff));
            }
        }
        None
    }

    fn impossible_sums(&self, max: usize) -> Vec<usize> {
        let mut nums = vec![];
        for i in 1..=max {
            if self.abundant_sum(i).is_none() {
                nums.push(i);
            }
            //println!("finished {i}, {}%", ((i as f32) / (max as f32) * 100_f32));
        }
        nums
    }
}

fn divisors(num: usize) -> HashSet<usize> {
    let max = (num as f32).sqrt().floor() as usize;
    let mut divisors = HashSet::from([1]);
    for i in 2..=max {
        if num % i == 0 {
            divisors.insert(i);
            divisors.insert(num / i);
        }
    }
    divisors
}

fn main() {
    let nums = PerfectNums::calc_nums(MAX_IMPOSSIBLE);
    let impossible = nums.impossible_sums(MAX_IMPOSSIBLE);
    let sum: usize = impossible.iter().sum();
    println!("{}", sum);
}

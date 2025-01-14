use std::collections::{HashMap, HashSet};

const MAX_NUM: usize = 10000;

fn divisors(n: usize) -> Vec<usize> {
    let mut divs = vec![1];
    let max = (n as f32).sqrt().floor() as usize;
    for i in 2..max {
        if n % i == 0 {
            divs.push(i);
            divs.push(n / i);
        }
    }
    divs.sort();
    divs
}

fn calc_divisors() -> HashMap<usize, Vec<usize>> {
    let mut div_map = HashMap::new();
    for i in 1..MAX_NUM {
        div_map.insert(i, divisors(i));
    }
    div_map
}

fn find_amicable(divs: HashMap<usize, Vec<usize>>) -> HashSet<usize> {
    let mut amicable = HashSet::new();
    for (num, divisors) in divs.iter() {
        if amicable.contains(num) {
            continue;
        }
        let sum: usize = divisors.iter().sum();
        if sum == *num {
            continue;
        }
        let sum_divisors = if let Some(divs2) = divs.get(&sum) {
            divs2
        } else {
            continue;
        };
        let sum2: usize = sum_divisors.iter().sum();
        if *num == sum2 {
            amicable.insert(*num);
            amicable.insert(sum);
        }
    }
    amicable
}

fn main() {
    let div_map = calc_divisors();
    let amicable = find_amicable(div_map);
    let sum: usize = amicable.iter().sum();
    println!("{sum:?}");
}

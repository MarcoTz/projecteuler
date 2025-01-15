use std::collections::{HashMap, HashSet};

fn num_digit_combinations() -> HashSet<(usize, usize, usize)> {
    let mut combinations = HashSet::new();
    for i in 1..=9 {
        for j in 1..=9 {
            for k in 1..=9 {
                if i + j + k != 9 {
                    continue;
                }
                if i <= k && j <= k && k <= j + i && !combinations.contains(&(j, i, k)) {
                    combinations.insert((i, j, k));
                }
                if i <= j && k <= j && j <= i + k && !combinations.contains(&(k, i, j)) {
                    combinations.insert((i, k, j));
                }
                if j <= i && k <= i && i <= j + k && !combinations.contains(&(k, j, i)) {
                    combinations.insert((j, k, i));
                }
            }
        }
    }
    combinations
}

fn nums_digits(num_digits: usize, cache: &mut HashMap<usize, Vec<usize>>) -> Vec<usize> {
    if let Some(nums) = cache.get(&num_digits) {
        return nums.clone();
    }
    let nums: Vec<usize> =
        (10_usize.pow(num_digits as u32 - 1)..10_usize.pow(num_digits as u32)).collect();
    cache.insert(num_digits, nums.clone());
    nums
}

fn digits(num: usize) -> Vec<usize> {
    let mut num = num;
    let mut digits = vec![];
    while num >= 10 {
        digits.push(num % 10);
        num = num / 10;
    }
    digits.push(num);
    digits.reverse();
    digits
}

fn main() {
    let combinations = num_digit_combinations();
    let digit_set = HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    println!(
        "got combinations\n{}",
        combinations
            .iter()
            .map(|comb| format!("{},{},{}", comb.0, comb.1, comb.2))
            .collect::<Vec<String>>()
            .join("\n")
    );

    let mut solutions = vec![];
    let mut cache = HashMap::new();
    for (num_a, num_b, num_c) in combinations {
        let candidates_a = nums_digits(num_a, &mut cache);
        let candidates_b = nums_digits(num_b, &mut cache);
        println!("got candidates for combination {num_a},{num_b},{num_c}");
        for a in candidates_a.iter() {
            for b in candidates_b.iter() {
                let prod = a * b;
                let prod_digits = digits(prod);

                if prod_digits.len() != num_c {
                    continue;
                }
                let mut all_digits = prod_digits;
                all_digits.extend(digits(*a));
                all_digits.extend(digits(*b));
                if all_digits.len() != 9 {
                    continue;
                }
                if digit_set != all_digits.into_iter().collect() {
                    continue;
                }
                solutions.push((*a, *b, prod));
            }
        }
        println!("finished combination {num_a},{num_b},{num_c}");
    }
    println!(
        "found solutions:\n{}",
        solutions
            .iter()
            .map(|solution| format!("{}*{}={}", solution.0, solution.1, solution.2))
            .collect::<Vec<String>>()
            .join("\n")
    );
    let sums: Vec<usize> = solutions.iter().map(|sol| sol.0 + sol.1 + sol.2).collect();
    println!(
        "sums for products:\n{}",
        sums.iter()
            .map(|sum| sum.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
    println!("total sum: {}", sums.iter().sum::<usize>());
}

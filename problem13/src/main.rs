use std::fs::read_to_string;

const NUMS_FILE: &str = "nums.txt";

mod num;
use num::Num;

fn load_nums() -> Vec<Num> {
    let contents = read_to_string(NUMS_FILE).unwrap();
    let mut nums = vec![];
    for line in contents.lines() {
        let mut num = vec![];
        for ch in line.chars() {
            num.push(ch.to_string().parse::<usize>().unwrap())
        }
        nums.push(Num { digits: num });
    }
    nums
}

fn main() {
    let nums = load_nums();
    let mut sum: Num = 0.into();
    for num in nums {
        sum = sum + num
    }
    println!("{}", sum.to_string().chars().take(10).collect::<String>());
}

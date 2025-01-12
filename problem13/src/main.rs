use std::{collections::VecDeque, fmt, fs::read_to_string};

const NUMS_FILE: &str = "nums.txt";
const NUM_DIGITS: usize = 50;

#[derive(Debug)]
pub struct Num<const N: usize> {
    pub digits: [usize; N],
}

impl<const N: usize> fmt::Display for Num<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.digits
                .iter()
                .map(|dig| dig.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

fn load_nums() -> Vec<Num<NUM_DIGITS>> {
    let contents = read_to_string(NUMS_FILE).unwrap();
    let mut nums = vec![];
    for line in contents.lines() {
        let mut num = [0; NUM_DIGITS];
        for (ind, ch) in line.chars().enumerate() {
            num[ind] = ch.to_string().parse().unwrap();
        }
        nums.push(Num { digits: num });
    }
    nums
}

fn sum_digits(nums: Vec<Num<NUM_DIGITS>>) -> Vec<usize> {
    let mut carry = 0;
    let mut sum_digits = VecDeque::new();
    for i in 0..NUM_DIGITS {
        let mut next_digit = 0;
        for num in nums.iter() {
            next_digit += num.digits[i];
        }
        next_digit += carry;
        carry = next_digit;
        next_digit = next_digit % 10;
        sum_digits.push_front(next_digit);
    }
    for dig in carry.to_string().chars().rev() {
        sum_digits.push_front(dig.to_string().parse::<usize>().unwrap())
    }
    sum_digits.into()
}

fn main() {
    let sum = sum_digits(load_nums());
    println!(
        "{}:{}",
        sum.len(),
        sum.iter()
            .map(|dig| dig.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
}

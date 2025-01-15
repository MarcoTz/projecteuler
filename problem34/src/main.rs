//from 10^7 on 10^n>n*9!
const MAX_NUM: usize = 1000000;
// 4 is the lowest number with n!>10 so the digit sum is not just n!
const MIN_NUM: usize = 4;

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

fn fac(n: usize) -> usize {
    if n == 1 || n == 0 {
        return 1;
    }
    n * fac(n - 1)
}

fn digits_fac(num: usize) -> usize {
    let digits = digits(num);
    let mut sum = 0;
    for digit in digits {
        sum += fac(digit);
    }
    sum
}

fn main() {
    let mut nums = vec![];
    for i in MIN_NUM..=MAX_NUM {
        let digit_sum = digits_fac(i);
        if i == digit_sum {
            nums.push(i)
        }
    }
    println!("{}", nums.iter().sum::<usize>());
}

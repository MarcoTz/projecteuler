const MAX_NUM: usize = 1000000;

fn num_to_digits(num: usize) -> Vec<usize> {
    let mut digits = vec![];
    let mut num = num;
    while num >= 10 {
        digits.push(num % 10);
        num = num / 10;
    }
    digits.push(num);
    digits.reverse();
    digits
}

fn digit_power_sum(num: usize, power: u32) -> usize {
    let mut sum = 0;
    let digits = num_to_digits(num);
    for digit in digits {
        sum += digit.pow(power);
    }
    sum
}

fn main() {
    let mut sum = 0;
    for i in 1..=MAX_NUM {
        let digit_sum = digit_power_sum(i, 5);
        if i == digit_sum {
            sum += digit_sum;
        }
    }
    println!("{}", sum)
}

const MAX_POWER: usize = 1000;

fn main() {
    let mut digits: Vec<usize> = vec![2];
    for _ in 2..=MAX_POWER {
        let mut carry = 0;
        for i in 0..digits.len() {
            let ind = digits.len() - 1 - i;
            let next_digit = digits[ind] * 2 + carry;
            carry = next_digit / 10;
            digits[ind] = next_digit % 10;
        }
        if carry == 0 {
            continue;
        }
        for new_digit in carry.to_string().chars().rev() {
            digits.insert(0, new_digit.to_string().parse::<usize>().unwrap());
        }
    }
    println!(
        "Sum of 2^{MAX_POWER}: {}",
        digits.iter().fold(0, |sum, next| sum + next)
    )
}

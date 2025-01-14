use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Number {
    digits: Vec<usize>,
}

impl Default for Number {
    fn default() -> Number {
        Number { digits: vec![1] }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(
            &self
                .digits
                .iter()
                .map(|digit| digit.to_string())
                .collect::<Vec<String>>()
                .join(""),
        )
    }
}

fn factorial(num: usize) -> Number {
    if num == 0 {
        return Number::default();
    }
    let fac = factorial(num - 1);
    let mut carry = 0;
    let mut new_num = Number { digits: vec![] };
    for digit in fac.digits.iter().rev() {
        let new_digit = num * digit + carry;
        new_num.digits.insert(0, new_digit % 10);
        carry = new_digit / 10;
    }
    while carry != 0 {
        let next_digit = carry % 10;
        carry -= next_digit;
        new_num.digits.insert(0, next_digit);
        if carry >= 10 {
            carry = carry / 10;
        }
    }
    new_num
}

fn main() {
    let fac = factorial(100);
    println!("{}", fac.digits.iter().fold(0, |sum, next| sum + next));
}

const MAX_NUM: u8 = 100;

#[derive(Debug, PartialEq, Eq)]
pub struct Number {
    digits: Vec<u8>,
}

impl Number {
    pub fn digit_sum(&self) -> u128 {
        self.digits.iter().map(|dig| *dig as u128).sum()
    }

    pub fn times(self, num: u8) -> Self {
        let mut digits = self.digits;
        digits.reverse();
        let mut new_digits = vec![];
        let mut rest: u128 = 0;
        for digit in digits {
            let new_digit = digit as u128 * num as u128 + rest;
            rest = new_digit / 10;
            new_digits.push((new_digit % 10) as u8);
        }
        while rest != 0 {
            new_digits.push((rest % 10) as u8);
            rest /= 10;
        }
        new_digits.reverse();
        Self { digits: new_digits }
    }
}

impl From<u8> for Number {
    fn from(num: u8) -> Self {
        if num < 10 {
            return Self {
                digits: Vec::from([num]),
            };
        }
        let mut number: Number = (num / 10).into();
        number.digits.push(num % 10);
        number
    }
}

fn main() {
    let mut max_sum = 0;
    for a in 1..MAX_NUM {
        let mut a_num: Number = a.into();
        for _ in 1..MAX_NUM {
            a_num = a_num.times(a);
            let sum = a_num.digit_sum();
            max_sum = max_sum.max(sum);
        }
    }
    println!("Max Digit Sum: {max_sum}");
}

#[cfg(test)]
mod number_tests {
    use super::Number;

    #[test]
    fn to_number1() {
        let num: Number = 12.into();
        let expected = Number { digits: vec![1, 2] };
        assert_eq!(num, expected)
    }

    #[test]
    fn to_number2() {
        let num: Number = 123.into();
        let expected = Number {
            digits: vec![1, 2, 3],
        };
        assert_eq!(num, expected)
    }

    #[test]
    fn times_2() {
        let num: Number = 12.into();
        let expected: Number = 24.into();
        assert_eq!(num.times(2), expected)
    }

    #[test]
    fn times_5() {
        let num: Number = 50.into();
        let expected: Number = 250.into();
        assert_eq!(num.times(5), expected)
    }
}

use std::{fmt, ops::Add};

#[derive(Clone)]
pub struct Num {
    pub digits: Vec<usize>,
}

impl Add for Num {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let mut self_digits = self.digits;
        let mut other_digits = other.digits;
        self_digits.reverse();
        other_digits.reverse();

        let mut carry = 0;
        let mut new_digits = vec![];
        while !self_digits.is_empty() && !other_digits.is_empty() {
            let mut new_digit = self_digits.remove(0);
            new_digit += other_digits.remove(0);
            new_digit += carry;
            new_digits.insert(0, new_digit % 10);
            carry = new_digit / 10;
        }
        let mut remaining_digits = if !self_digits.is_empty() {
            self_digits
        } else {
            other_digits
        };
        while !remaining_digits.is_empty() {
            let mut new_digit = remaining_digits.remove(0);
            new_digit += carry;
            new_digits.insert(0, new_digit % 10);
            carry = new_digit / 10;
        }

        while carry >= 10 {
            new_digits.insert(0, carry % 10);
            carry = carry / 10;
        }
        if carry != 0 {
            new_digits.insert(0, carry);
        }

        Num { digits: new_digits }
    }
}

impl From<usize> for Num {
    fn from(n: usize) -> Num {
        let mut num = n;
        let mut digits = vec![];
        while num >= 10 {
            digits.insert(0, num % 10);
            num = num / 10;
        }
        if num != 0 {
            digits.insert(0, num)
        }
        Num { digits }
    }
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.digits
                .iter()
                .map(|digit| digit.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

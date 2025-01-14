use std::fmt;

const MAX_DENOMINATOR: usize = 1000;

#[derive(Default)]
struct Decimal {
    whole_part: usize,
    decimal_digits: Vec<usize>,
    recurring_digits: Vec<usize>,
}

impl Decimal {
    fn unit_fraction(denominator: usize) -> Decimal {
        let mut remainder = 10;
        let mut digits = vec![];
        while remainder != 0 {
            let next_digit = remainder / denominator;
            remainder = remainder - next_digit * denominator;
            remainder *= 10;
            if digits.contains(&next_digit) {
                let index = digits
                    .iter()
                    .position(|digit| *digit == next_digit)
                    .unwrap();
                let recurring = digits.split_off(index);
                return Decimal {
                    whole_part: 0,
                    decimal_digits: digits,
                    recurring_digits: recurring,
                };
            }
            digits.push(next_digit);
        }
        Decimal {
            whole_part: 0,
            decimal_digits: digits,
            recurring_digits: vec![],
        }
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rec_part = if self.recurring_digits.is_empty() {
            "".to_owned()
        } else {
            format!(
                "({})",
                self.recurring_digits
                    .iter()
                    .map(|dig| dig.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            )
        };

        write!(
            f,
            "{}.{}{}",
            self.whole_part,
            self.decimal_digits
                .iter()
                .map(|dig| dig.to_string())
                .collect::<Vec<String>>()
                .join(""),
            rec_part
        )
    }
}

fn main() {
    let mut max_cycle = Decimal::default();
    let mut max_n = 1;
    for i in 2..=MAX_DENOMINATOR {
        let next_dec = Decimal::unit_fraction(i);
        if max_cycle.recurring_digits.len() < next_dec.recurring_digits.len() {
            max_n = i;
            max_cycle = next_dec;
        }
    }

    println!("longest cycle: 1/{max_n}: {max_cycle}");
}

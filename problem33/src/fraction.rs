use super::gcd;
use std::{
    fmt,
    ops::{Add, Mul, Neg},
};

#[derive(Debug, Copy, Clone)]
pub struct Fraction {
    numerator: i64,
    denominator: usize,
}

impl Fraction {
    pub fn new(numerator: i64, denominator: usize) -> Option<Fraction> {
        if denominator == 0 {
            return None;
        }
        Some(
            Fraction {
                numerator,
                denominator,
            }
            .simplify(),
        )
    }

    fn simplify(self) -> Fraction {
        let gcd = gcd(self.numerator.abs() as usize, self.denominator);
        if gcd == 0 {
            return self;
        }
        Fraction {
            numerator: self.numerator / (gcd as i64),
            denominator: self.denominator / gcd,
        }
    }
}

impl Add for Fraction {
    type Output = Self;
    fn add(self, other: Fraction) -> Self::Output {
        let new_denominator = self.denominator * other.denominator;
        let self_numerator_extended = self.numerator * (other.denominator as i64);
        let other_numerator_extended = other.numerator * (self.denominator as i64);
        Fraction {
            numerator: self_numerator_extended + other_numerator_extended,
            denominator: new_denominator,
        }
        .simplify()
    }
}

impl Mul for Fraction {
    type Output = Self;
    fn mul(self, other: Fraction) -> Self::Output {
        Fraction {
            numerator: self.numerator * other.numerator,
            denominator: self.denominator * other.denominator,
        }
        .simplify()
    }
}

impl Neg for Fraction {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Fraction {
            numerator: self.numerator.neg(),
            denominator: self.denominator,
        }
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Fraction) -> bool {
        let self_simple = self.simplify();
        let other_simple = other.simplify();
        self_simple.numerator.eq(&other_simple.numerator)
            && self_simple.denominator.eq(&other_simple.denominator)
    }
}

impl Eq for Fraction {}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

#[cfg(test)]
mod fraction_tests {
    use super::Fraction;

    fn example_a() -> Fraction {
        Fraction::new(5, 7).unwrap()
    }

    fn example_b() -> Fraction {
        Fraction::new(-1, 12).unwrap()
    }

    #[test]
    fn a_neg() {
        let result = -example_a();
        let expected = Fraction::new(-5, 7).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn b_neg() {
        let result = -example_b();
        let expected = Fraction::new(1, 12).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn a_plus_b() {
        let result = example_a() + example_b();
        let expected = Fraction::new(53, 84).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn a_times_b() {
        let result = example_a() * example_b();
        let expected = Fraction::new(-5, 84).unwrap();
        assert_eq!(result, expected)
    }
}

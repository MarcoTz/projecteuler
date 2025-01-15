mod fraction;
use fraction::Fraction;

const MAX_NUM: usize = 99;
const MAX_DENOM: usize = 99;

pub fn gcd(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        return 0;
    }
    if a < b {
        return gcd(b, a);
    }

    let rem = a % b;
    if rem == 0 {
        return b;
    }
    gcd(b, rem)
}

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

fn digits_to_num(digits: Vec<usize>) -> usize {
    let mut num = 0;
    for (ind, digit) in digits.into_iter().rev().enumerate() {
        num += 10_usize.pow(ind as u32) * digit;
    }
    num
}

fn main() {
    let mut fracs = vec![];
    for numerator in 10..=MAX_NUM {
        for denominator in 10..=MAX_DENOM {
            if (numerator / 10) * 10 == numerator && (denominator / 10) * 10 == denominator {
                continue;
            }
            if numerator > denominator {
                continue;
            }

            let frac = Fraction::new(numerator as i64, denominator).unwrap();

            let num_digits = digits(numerator);
            let denom_digits = digits(denominator);

            let mut digits_num_simpl = vec![];
            let mut digits_denom_simpl = vec![];
            for digit in num_digits.iter() {
                if !denom_digits.contains(digit) {
                    digits_num_simpl.push(*digit);
                }
            }
            if digits_num_simpl == num_digits {
                continue;
            }
            let num_simpl = digits_to_num(digits_num_simpl);

            for digit in denom_digits.iter() {
                if !num_digits.contains(digit) {
                    digits_denom_simpl.push(*digit);
                }
            }
            if denom_digits == digits_denom_simpl {
                continue;
            }
            let denom_simpl = digits_to_num(digits_denom_simpl);
            if denom_simpl == 0 {
                continue;
            }
            let frac_simpl = Fraction::new(num_simpl as i64, denom_simpl).unwrap();
            if frac == frac_simpl {
                fracs.push(frac)
            }
        }
    }
    let prod = fracs
        .into_iter()
        .fold(Fraction::new(1, 1).unwrap(), |prod, next| prod * next);
    println!("{}", prod);
}

#[cfg(test)]
mod gcd_tests {
    use super::gcd;

    #[test]
    fn gcd_coprime() {
        let result = gcd(15, 23);
        let expected = 1;
        assert_eq!(result, expected)
    }

    #[test]
    fn gcd_fifty() {
        let result = gcd(250, 100);
        let expected = 50;
        assert_eq!(result, expected)
    }

    #[test]
    fn gcd_two() {
        let result = gcd(6, 8);
        let expected = 2;
        assert_eq!(result, expected)
    }
}

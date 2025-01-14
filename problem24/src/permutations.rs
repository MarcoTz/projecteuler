use super::permutation::Permutation;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Permutations {
    digits: Vec<usize>,
    pub permutations: Vec<Permutation>,
}

impl Permutations {
    pub fn new(digits: Vec<usize>) -> Permutations {
        if digits.is_empty() {
            return Permutations {
                digits: vec![],
                permutations: vec![],
            };
        }
        let mut new_digits = digits.clone();
        let first = new_digits.remove(0);

        if new_digits.is_empty() {
            return Permutations {
                digits: vec![first],
                permutations: vec![vec![first].into()],
            };
        }

        let lower_perm = Permutations::new(new_digits);
        let mut new_perms = vec![];
        for perm in lower_perm.permutations {
            for i in 0..=perm.len() {
                let mut new_perm = perm.clone();
                new_perm.insert(i, first);
                new_perms.push(new_perm);
            }
        }
        println!("generated permutations for {digits:?}");
        Permutations {
            digits,
            permutations: new_perms,
        }
    }

    pub fn from_max(max: usize) -> Permutations {
        Permutations::new((1..=max).collect())
    }

    pub fn sort(&mut self) {
        println!("sorting permutations");
        self.permutations.sort()
    }
}

impl fmt::Display for Permutations {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{}",
            self.digits
                .iter()
                .map(|dig| dig.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        for permutation in self.permutations.iter() {
            writeln!(f, "{}", permutation)?;
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod permutations_tests {
    use super::Permutations;
    use crate::permutation::permutation_tests::perms;

    #[test]
    fn test_sort() {
        let mut result = Permutations::from_max(3);
        result.sort();
        let expected = Permutations {
            digits: vec![1, 2, 3],
            permutations: perms(),
        };
        assert_eq!(result, expected)
    }
}

use std::{cmp::Ordering, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Permutation(Vec<usize>);

impl Permutation {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, ind: usize, elem: usize) {
        self.0.insert(ind, elem)
    }
}

impl fmt::Display for Permutation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(
            &self
                .0
                .iter()
                .map(|dig| dig.to_string())
                .collect::<Vec<String>>()
                .join(" "),
        )
    }
}

impl PartialOrd for Permutation {
    fn partial_cmp(&self, other: &Permutation) -> Option<Ordering> {
        for (i, j) in self.0.iter().zip(other.0.iter()) {
            match i.cmp(j) {
                Ordering::Less => {
                    return Some(Ordering::Less);
                }
                Ordering::Greater => {
                    return Some(Ordering::Greater);
                }
                Ordering::Equal => {
                    continue;
                }
            }
        }
        if self.0.len() == other.0.len() {
            return Some(Ordering::Equal);
        }
        if self.0.len() < other.0.len() {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl Ord for Permutation {
    fn cmp(&self, other: &Permutation) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl From<Vec<usize>> for Permutation {
    fn from(perm: Vec<usize>) -> Permutation {
        Permutation(perm)
    }
}

#[cfg(test)]
pub mod permutation_tests {
    use super::Permutation;

    pub fn perms() -> Vec<Permutation> {
        vec![
            Permutation(vec![1, 2, 3]),
            Permutation(vec![1, 3, 2]),
            Permutation(vec![2, 1, 3]),
            Permutation(vec![2, 3, 1]),
            Permutation(vec![3, 1, 2]),
            Permutation(vec![3, 2, 1]),
        ]
    }

    #[test]
    fn cmp_test() {
        let perms = perms();
        assert!(perms[0] < perms[1]);
        assert!(perms[1] < perms[2]);
        assert!(perms[2] < perms[3]);
        assert!(perms[3] < perms[4]);
        assert!(perms[4] < perms[5]);
    }

    #[test]
    fn display_test() {
        let result = Permutation(vec![1, 2, 3]).to_string();
        let expected = "1 2 3";
        assert_eq!(result, expected)
    }
}

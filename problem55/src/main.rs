const MAX_TEST: u128 = 10000;
const MAX_ITERS: u128 = 50;

fn lyrchel_iter(i: u128) -> u128 {
    i + rev(i)
}

fn rev(i: u128) -> u128 {
    let i_rev_str = i
        .to_string()
        .chars()
        .rev()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("");
    i_rev_str.parse().unwrap()
}

fn is_palindrome(i: u128) -> bool {
    return i == rev(i);
}

fn test_lyrchel(i: u128) -> bool {
    let mut curr = i;
    for _ in 0..MAX_ITERS {
        curr = lyrchel_iter(curr);
        if is_palindrome(curr) {
            return false;
        }
    }
    true
}

fn main() {
    let mut num_lyrchel = 0;
    for i in 1..MAX_TEST {
        if test_lyrchel(i) {
            num_lyrchel += 1;
        }
    }

    println!("There are {num_lyrchel} Lyrchel Numbers below {MAX_TEST}");
}

#[cfg(test)]
mod lyrchel_tests {
    use super::{is_palindrome, test_lyrchel};

    #[test]
    fn test_palindrome1() {
        let num = 11211;
        assert!(is_palindrome(num));
    }

    #[test]
    fn test_palindrome2() {
        let num = 3456543;
        assert!(is_palindrome(num));
    }

    #[test]
    fn test_no_palindrome1() {
        let num = 123123;
        assert!(!is_palindrome(num));
    }

    #[test]
    fn test_no_palindrome2() {
        let num = 1948136;
        assert!(!is_palindrome(num));
    }

    #[test]
    fn test_lyrchel1() {
        let num = 47;
        assert!(!test_lyrchel(num));
    }

    #[test]
    fn test_no_lyrchel() {
        let num = 196;
        assert!(test_lyrchel(num))
    }
}

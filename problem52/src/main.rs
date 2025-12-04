use std::collections::HashSet;

fn char_to_digit(ch: char) -> u8 {
    match ch {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => panic!("Not a valid digit"),
    }
}

fn digits(num: u64) -> HashSet<u8> {
    num.to_string().chars().map(char_to_digit).collect()
}

fn test_digits(num: u64) -> bool {
    let num_digits = digits(num);
    for i in 2..=6 {
        let new_digits = digits(num * i);
        if new_digits != num_digits {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut curr = 1;
    loop {
        if test_digits(curr) {
            break;
        }
        curr += 1;
    }
    println!("Found number: {curr}");
}

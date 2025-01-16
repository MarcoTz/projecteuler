use std::fs::read_to_string;

const NAMES_FILE: &str = "names.txt";

fn string_to_num(s: String) -> Vec<usize> {
    let mut nums = vec![];
    for c in s.chars() {
        if c == ' ' {
            continue;
        }
        nums.push(char_to_num(c));
    }
    nums
}

fn char_to_num(c: char) -> usize {
    match c.to_lowercase().next().unwrap() {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        c => panic!("Not an alphabetic character {c}"),
    }
}

fn get_score(name: String, index: usize) -> usize {
    let nums = string_to_num(name);
    let sum: usize = nums.iter().sum();
    (index + 1) * sum
}

fn calculate_scores(names: Vec<String>) -> usize {
    let mut total = 0;
    for (ind, name) in names.into_iter().enumerate() {
        total += get_score(name, ind);
    }
    total
}

fn load_names() -> Vec<String> {
    let mut names = vec![];
    let names_str = read_to_string(NAMES_FILE).unwrap();
    for name in names_str.split(",") {
        names.push(name.trim().replace('"', ""));
    }
    names
}

fn main() {
    let mut names = load_names();
    names.sort();
    println!("{}", calculate_scores(names));
}

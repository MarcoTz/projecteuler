const MAX_NUM: usize = 1000000;

fn num_to_bin(num: usize) -> String {
    format!("{num:b}")
}

fn is_palindrome(s: String) -> bool {
    let mut chars: Vec<char> = s.chars().collect();
    let mid = chars.len() / 2;
    if chars.len() % 2 == 1 {
        chars.remove(mid);
    }
    let (beginning, end) = chars.split_at(mid);
    let beginning: Vec<&char> = beginning.iter().collect();
    let end: Vec<&char> = end.iter().rev().collect();
    beginning == end
}

fn main() {
    let mut palindromes = vec![];
    for i in 1..=MAX_NUM {
        if !is_palindrome(i.to_string()) {
            continue;
        }
        if is_palindrome(num_to_bin(i)) {
            palindromes.push(i);
        }
    }
    println!(
        "{}",
        palindromes
            .iter()
            .map(|num| format!("{num}=0x{num:b}"))
            .collect::<Vec<String>>()
            .join("\n")
    );
    println!("sum: {}", palindromes.iter().sum::<usize>())
}

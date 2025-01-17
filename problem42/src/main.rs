use std::fs::read_to_string;

mod triangle;
use triangle::TriangleGen;

const WORD_FILE: &str = "words.txt";

fn char2num(c: char) -> u8 {
    (c.to_ascii_uppercase() as u8) - 64
}

fn word2num(word: String) -> u64 {
    word.chars()
        .into_iter()
        .map(|c| char2num(c) as u64)
        .sum::<u64>()
}

fn load_words() -> Vec<String> {
    let contents = read_to_string(WORD_FILE).unwrap();
    let mut words = vec![];
    for word in contents.split(",") {
        words.push(word.replace('\"', ""));
    }
    words
}

fn main() {
    let words = load_words();
    let mut gen = TriangleGen::new();
    let triangle_words = words
        .into_iter()
        .map(word2num)
        .filter(|num| gen.is_triangle(*num))
        .count();
    println!("{}", triangle_words);
}

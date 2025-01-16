fn say_num(num: usize) -> String {
    match num {
        0 => "zero".to_owned(),
        1 => "one".to_owned(),
        2 => "two".to_owned(),
        3 => "three".to_owned(),
        4 => "four".to_owned(),
        5 => "five".to_owned(),
        6 => "six".to_owned(),
        7 => "seven".to_owned(),
        8 => "eight".to_owned(),
        9 => "nine".to_owned(),
        10 => "ten".to_owned(),
        11 => "eleven".to_owned(),
        12 => "twelve".to_owned(),
        13 => "thirteen".to_owned(),
        14 => "fourteen".to_owned(),
        15 => "fifteen".to_owned(),
        16 => "sixteen".to_owned(),
        17 => "seventeen".to_owned(),
        18 => "eighteen".to_owned(),
        19 => "nineteen".to_owned(),
        i if i < 100 => {
            let tenth_str = say_tenth(i / 10);
            let unit_str = if i % 10 == 0 {
                "".to_owned()
            } else {
                say_num(i % 10)
            };
            format!("{tenth_str}{unit_str}")
        }
        i => {
            let end = if i % 100 == 0 {
                "".to_owned()
            } else {
                format!("and{}", say_num(i % 100))
            };
            let start = say_num(i / 100);
            format!("{start}hundred{end}")
        }
    }
}

fn say_tenth(num: usize) -> String {
    match num {
        1 => "teen".to_owned(),
        2 => "twenty".to_owned(),
        3 => "thirty".to_owned(),
        4 => "forty".to_owned(),
        5 => "fifty".to_owned(),
        6 => "sixty".to_owned(),
        7 => "seventy".to_owned(),
        8 => "eighty".to_owned(),
        9 => "ninety".to_owned(),
        _ => panic!("Number too high"),
    }
}

fn main() {
    let mut count = 0;
    for i in 1..1000 {
        count += say_num(i).len();
    }
    count += "onethousand".len();
    println!("Number of letters {count}");
}

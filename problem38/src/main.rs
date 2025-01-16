use std::collections::HashSet;

#[derive(Debug)]
struct NumProduct {
    num: usize,
    max_n: usize,
    concated: String,
}

fn product(num: usize) -> Option<NumProduct> {
    let mut product = num.to_string();
    let mut n = 2;
    loop {
        let next_product = product.clone() + &(n * num).to_string();
        if next_product.len() > 9 {
            break;
        }
        product = next_product;
        n += 1;
    }
    if product.len() == 9 {
        Some(NumProduct {
            num,
            max_n: n - 1,
            concated: product,
        })
    } else {
        None
    }
}

fn num_digits(num: usize) -> Vec<usize> {
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

fn main() {
    let digits = HashSet::from(['1', '2', '3', '4', '5', '6', '7', '8', '9']);
    let mut num = 1;
    let mut pandigitals = vec![];
    while num_digits(num).len() < 9 {
        let res = product(num);
        num += 1;
        if let Some(prod) = res {
            if prod.concated.chars().collect::<HashSet<char>>() == digits {
                pandigitals.push(prod);
            }
        }
    }

    let max_num = pandigitals
        .iter()
        .max_by(|prod, other| prod.num.cmp(&other.num))
        .unwrap();

    println!(
        "max: {}\n{}\n{}",
        max_num.num,
        max_num.concated,
        (1..=max_num.max_n)
            .map(|i| format!("{i}*{}={}", max_num.num, (i * max_num.num)))
            .collect::<Vec<String>>()
            .join("\n")
    );
}

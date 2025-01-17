const NUM_DIGITS: u64 = 10_u64.pow(10);
const MAX_NUM: u64 = 1000;

fn mod_power(num: u64, power: u64, modulo: u64) -> u64 {
    let mut power = power - 1;
    let mut res = num;
    while power > 0 {
        res *= num;
        if res > modulo {
            res = res % modulo
        }
        power -= 1;
    }
    res % modulo
}

fn main() {
    let mut sum = 0;
    for i in 1..=MAX_NUM {
        sum += mod_power(i, i, NUM_DIGITS);
        sum = sum % NUM_DIGITS;
    }
    println!("{sum}");
}

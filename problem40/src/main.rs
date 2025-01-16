const MAX_TEN_POW: u32 = 6;

struct Champerone {
    pos: u64,
    current_num: u64,
    digit_buffer: Vec<u8>,
}

impl Default for Champerone {
    fn default() -> Champerone {
        Champerone {
            pos: 1,
            current_num: 0,
            digit_buffer: vec![],
        }
    }
}

impl Iterator for Champerone {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        self.pos += 1;
        if !self.digit_buffer.is_empty() {
            return Some(self.digit_buffer.remove(0));
        }

        self.current_num += 1;
        let mut digits_num = digits(self.current_num);
        let item = Some(digits_num.remove(0));
        self.digit_buffer.extend(digits_num);
        item
    }
}

fn digits(num: u64) -> Vec<u8> {
    let mut num = num;
    let mut digits = vec![];
    while num >= 10 {
        digits.push((num % 10) as u8);
        num = num / 10;
    }

    digits.push(num as u8);
    digits.reverse();
    digits
}

fn main() {
    let champ = Champerone::default();
    let first_n: Vec<u8> = champ.take(10_usize.pow(MAX_TEN_POW)).collect();
    let mut prod = 1;
    for i in 0..=MAX_TEN_POW {
        let index = 10_usize.pow(i) - 1;
        let next = first_n[index];
        prod *= next
    }

    println!("Product : {prod}");
}

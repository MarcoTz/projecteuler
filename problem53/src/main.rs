use std::collections::HashMap;

// (n r) + (n (r+1)) = (n+1 r+1)
// (n r) = (n-1 r-1) + (n-1 r)
fn n_choose_r(n: u8, r: u8, cache: &mut HashMap<(u8, u8), u128>) -> u128 {
    if let Some(res) = cache.get(&(n, r)) {
        return *res;
    }
    if r == 0 || n == 0 || r == n {
        return 1;
    }
    if r > n {
        return 0;
    }

    let res = n_choose_r(n - 1, r - 1, cache) + n_choose_r(n - 1, r, cache);
    cache.insert((n, r), res);
    res
}

fn main() {
    let mut cache = HashMap::new();
    let mut num_larger = 0;
    let mut larger = Vec::new();
    for n in 1..=100 {
        for r in 0..=n {
            let chosen = n_choose_r(n, r, &mut cache);
            if chosen > 1000000 {
                num_larger += 1;
                larger.push(chosen);
            }
        }
    }
    println!("Larger than one million: {num_larger}")
}

mod permutation;
mod permutations;
use permutations::Permutations;

fn main() {
    let mut perms = Permutations::from_max(10);
    perms.sort();
    println!("{}", perms.permutations[999999]);
}

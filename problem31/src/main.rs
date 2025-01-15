use std::{cmp::Ordering, collections::HashMap, fmt};

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
enum Coin {
    One,
    Two,
    Five,
    Ten,
    Twenty,
    Fifty,
    Pound,
    TwoPound,
}

impl Coin {
    fn all() -> Vec<Coin> {
        vec![
            Coin::One,
            Coin::Two,
            Coin::Five,
            Coin::Ten,
            Coin::Twenty,
            Coin::Fifty,
            Coin::Pound,
            Coin::TwoPound,
        ]
    }

    fn value(&self) -> usize {
        match self {
            Coin::One => 1,
            Coin::Two => 2,
            Coin::Five => 5,
            Coin::Ten => 10,
            Coin::Twenty => 20,
            Coin::Fifty => 50,
            Coin::Pound => 100,
            Coin::TwoPound => 100,
        }
    }
}

impl PartialOrd for Coin {
    fn partial_cmp(&self, other: &Coin) -> Option<Ordering> {
        Some(self.value().cmp(&other.value()))
    }
}

impl Ord for Coin {
    fn cmp(&self, other: &Coin) -> Ordering {
        self.value().cmp(&other.value())
    }
}

#[derive(Debug, Clone)]
struct CoinCombination(Vec<Coin>);

impl CoinCombination {
    fn make_val(
        val: usize,
        cache: &mut HashMap<usize, Vec<CoinCombination>>,
    ) -> Vec<CoinCombination> {
        if val == 0 {
            return vec![];
        }

        if let Some(combinations) = cache.get(&val) {
            return combinations.clone();
        }

        let coins = Coin::all();
        let possible: Vec<Coin> = coins
            .into_iter()
            .filter(|coin| coin.value() <= val)
            .collect();
        let mut combinations: Vec<CoinCombination> = vec![];
        for coin in possible {
            let coin_val = coin.value();
            if coin_val == val {
                combinations.push(CoinCombination(vec![coin]));
                continue;
            }

            let new_val = val - coin_val;
            let new_combinations = CoinCombination::make_val(new_val, cache);
            for mut combination in new_combinations {
                combination.0.push(coin.clone());
                if combinations
                    .iter()
                    .find(|comb| **comb == combination)
                    .is_some()
                {
                    continue;
                }
                combinations.push(combination);
            }
        }
        cache.insert(val, combinations.clone());
        println!("calculated combinations for {val}");
        combinations
    }
}

impl PartialEq for CoinCombination {
    fn eq(&self, other: &CoinCombination) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }

        let mut other_coins: Vec<&Coin> = other.0.iter().collect();
        for coin in self.0.iter() {
            if !other_coins.contains(&coin) {
                return false;
            }
            let ind = other_coins.iter().position(|c| *c == coin).unwrap();
            other_coins.remove(ind);
        }

        true
    }
}

impl Eq for CoinCombination {}

impl PartialOrd for CoinCombination {
    fn partial_cmp(&self, other: &CoinCombination) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl Ord for CoinCombination {
    fn cmp(&self, other: &CoinCombination) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl fmt::Display for CoinCombination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|coin| format!("{:?}", coin))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

fn main() {
    let mut combinations = CoinCombination::make_val(200, &mut Default::default());
    //let _ = combinations.iter_mut().map(|comb| comb.0.sort());
    //combinations.sort();
    println!("{}", combinations.len());
}

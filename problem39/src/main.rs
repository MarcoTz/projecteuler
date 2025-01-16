use std::{collections::HashMap, fmt};

const MAX_PERI: u64 = 1000;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct PythagoreanTriple {
    a: u64,
    b: u64,
    c: u64,
}

impl PythagoreanTriple {
    fn generate(max: u64) -> Vec<Self> {
        let mut triples = vec![];
        let mut m = 0;
        loop {
            let min_peri = 2 * (m * m + m);
            if min_peri > max {
                break;
            }
            for n in 1..m {
                let a = m * m - n * n;
                let b = 2 * m * n;
                let c = m * m + n * n;
                let mut mult = 1;
                let mut new_triple = PythagoreanTriple { a, b, c };
                loop {
                    if new_triple.peri() > max {
                        break;
                    }
                    triples.push(new_triple);
                    new_triple.a *= mult;
                    new_triple.b *= mult;
                    new_triple.c *= mult;
                    mult += 1;
                }
            }
            m += 1;
        }
        triples
    }

    fn peri(&self) -> u64 {
        self.a + self.b + self.c
    }
}

impl fmt::Display for PythagoreanTriple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.a, self.b, self.c)
    }
}

fn bucket(triples: Vec<PythagoreanTriple>) -> HashMap<u64, Vec<PythagoreanTriple>> {
    let mut buckets: HashMap<u64, Vec<PythagoreanTriple>> = HashMap::new();
    for triple in triples {
        let peri = triple.peri();
        match buckets.get_mut(&peri) {
            Some(triples) => {
                triples.push(triple);
            }
            None => {
                buckets.insert(peri, vec![triple]);
            }
        }
    }
    buckets
}

fn main() {
    let triples = PythagoreanTriple::generate(MAX_PERI);
    let buckets = bucket(triples);
    let max_key = buckets
        .iter()
        .map(|(label, triples)| (label, triples.len()))
        .max_by(|(_, len1), (_, len2)| len1.cmp(len2))
        .unwrap()
        .0;
    let max_triples = buckets.get(max_key).unwrap();
    println!(
        "{}:{}\n{}",
        max_key,
        max_triples.len(),
        max_triples
            .iter()
            .map(|triple| triple.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
}

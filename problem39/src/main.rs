use std::{collections::HashMap, fmt};

const MAX_PERI: u64 = 1000;

#[derive(Debug, Clone, Copy)]
struct PythagoreanTriple {
    a: u64,
    b: u64,
    c: u64,
}

impl PythagoreanTriple {
    fn generate(max: u64) -> Vec<Self> {
        let mut triples = vec![];
        let mut m = 1;
        loop {
            let min_peri = 2 * (m * m + m);
            if min_peri > max {
                break;
            }
            for n in 1..m {
                let a = m * m - n * n;
                let b = 2 * m * n;
                let c = m * m + n * n;
                let mut new_triple = PythagoreanTriple { a, b, c };
                loop {
                    if new_triple.peri() > max {
                        break;
                    }
                    if !triples.contains(&new_triple) {
                        triples.push(new_triple);
                    }
                    new_triple.a += a;
                    new_triple.b += b;
                    new_triple.c += c;
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

impl PartialEq for PythagoreanTriple {
    fn eq(&self, other: &PythagoreanTriple) -> bool {
        (self.a == other.a && self.b == other.b) || (self.a == other.b && self.b == other.b)
    }
}

impl Eq for PythagoreanTriple {}

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

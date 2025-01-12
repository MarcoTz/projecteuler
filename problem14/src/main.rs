use std::collections::{HashMap, HashSet};

const MAX_NUM: usize = 1000000;

struct CollatzGraph {
    pub vertices: HashSet<usize>,
    pub edges: HashMap<usize, usize>,
}

impl CollatzGraph {
    pub fn new() -> CollatzGraph {
        CollatzGraph {
            vertices: HashSet::from([1]),
            edges: HashMap::new(),
        }
    }
}

impl CollatzGraph {
    pub fn add_num(&mut self, num: usize) {
        if self.vertices.contains(&num) {
            return;
        }
        let next = if num % 2 == 0 { num / 2 } else { 3 * num + 1 };
        self.vertices.insert(num);
        self.edges.insert(num, next);
        self.add_num(next);
    }

    pub fn chain(&mut self, num: usize) -> Vec<usize> {
        if num == 1 {
            return vec![];
        }

        self.add_num(num);
        let next = self.edges.get(&num).unwrap();
        let mut ch = self.chain(*next);
        ch.insert(0, num);
        ch
    }
}

fn main() {
    let mut collatz = CollatzGraph::new();
    struct MaxResult {
        num: usize,
        chain: Vec<usize>,
    }
    let mut longest_chain = MaxResult {
        num: 1,
        chain: vec![],
    };

    for i in 1..=MAX_NUM {
        let chain = collatz.chain(i);
        let max_len = longest_chain.chain.len();
        if chain.len() > max_len {
            longest_chain = MaxResult { num: i, chain };
        }
        println!("finished {i}, max len: {max_len}");
    }

    println!(
        "Longest Chain at {}, {}\n{:?}",
        longest_chain.num,
        longest_chain.chain.len(),
        longest_chain.chain
    );
}

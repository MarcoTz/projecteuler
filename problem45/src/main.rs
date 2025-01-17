struct TriangleGen {
    next_n: u64,
    generated: Vec<u64>,
}

impl TriangleGen {
    fn new() -> TriangleGen {
        TriangleGen {
            next_n: 1,
            generated: vec![],
        }
    }

    fn max_generated(&self) -> u64 {
        self.generated.iter().max().copied().unwrap_or(0)
    }
}

impl Iterator for TriangleGen {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.next_n * (self.next_n + 1) / 2;
        self.next_n += 1;
        self.generated.push(item);
        Some(item)
    }
}

struct PentagonGen {
    next_n: u64,
    generated: Vec<u64>,
}

impl PentagonGen {
    fn new() -> PentagonGen {
        PentagonGen {
            next_n: 1,
            generated: vec![],
        }
    }

    fn max_generated(&self) -> u64 {
        self.generated.iter().max().copied().unwrap_or(0)
    }
}

impl Iterator for PentagonGen {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.next_n * (3 * self.next_n - 1) / 2;
        self.next_n += 1;
        self.generated.push(item);
        Some(item)
    }
}

struct HexagonGen {
    next_n: u64,
    generated: Vec<u64>,
}

impl HexagonGen {
    fn new() -> HexagonGen {
        HexagonGen {
            next_n: 1,
            generated: vec![],
        }
    }

    fn max_generated(&self) -> u64 {
        self.generated.iter().max().copied().unwrap_or(0)
    }
}

impl Iterator for HexagonGen {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.next_n * (2 * self.next_n - 1);
        self.next_n += 1;
        self.generated.push(item);
        Some(item)
    }
}

fn main() {
    let mut triangle = TriangleGen::new();
    let mut pentagon = PentagonGen::new();
    let mut hexagon = HexagonGen::new();
    loop {
        let next_triangle = triangle.next().unwrap();
        while pentagon.max_generated() < next_triangle {
            pentagon.next();
        }
        if !pentagon.generated.contains(&next_triangle) {
            continue;
        }
        while hexagon.max_generated() < next_triangle {
            hexagon.next();
        }
        if !hexagon.generated.contains(&next_triangle) {
            continue;
        }

        println!("{}", next_triangle);
        if next_triangle > 40755 {
            break;
        }
    }
}

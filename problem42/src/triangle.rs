pub struct TriangleGen {
    next: u64,
    generated: Vec<u64>,
}

impl TriangleGen {
    pub fn new() -> TriangleGen {
        TriangleGen {
            next: 1,
            generated: vec![],
        }
    }

    pub fn nth(&mut self, n: usize) -> u64 {
        if self.generated.len() > n {
            return self.generated[n];
        }

        for _ in (self.next as usize)..=n {
            self.next();
        }
        *self.generated.last().unwrap()
    }

    pub fn is_triangle(&mut self, num: u64) -> bool {
        self.next();
        let mut biggest = *self.generated.last().unwrap();
        while biggest < num {
            biggest = self.next().unwrap();
        }
        self.generated.contains(&num)
    }
}

impl Iterator for TriangleGen {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let next = (self.next * (self.next + 1)) / 2;
        self.next += 1;
        self.generated.push(next);
        Some(next)
    }
}

use std::{fmt, fs::read_to_string, iter};

const TRIANGLE_FILE: &str = "triangle.txt";

type TriangleRow = Vec<u64>;

#[derive(Debug, Clone)]
struct Triangle {
    rows: Vec<TriangleRow>,
}

impl Triangle {
    fn load() -> Triangle {
        let contents = read_to_string(TRIANGLE_FILE).unwrap();
        let mut triangle = Triangle { rows: vec![] };
        for line in contents.lines() {
            let line_nums: TriangleRow = line
                .split(' ')
                .map(|num| num.parse::<u64>().unwrap())
                .collect();
            triangle.rows.push(line_nums);
        }
        triangle
    }
    fn get(&self, x: usize, y: usize) -> Option<u64> {
        let row = self.rows.get(y)?;
        row.get(x).copied()
    }

    fn max_sum(&self, start_x: usize, start_y: usize) -> u64 {
        let val = self.get(start_x, start_y).unwrap_or(0);
        if start_y == self.rows.len() - 1 || val == 0 {
            return val;
        }
        let left_sum = self.max_sum(start_x, start_y + 1);
        let right_sum = self.max_sum(start_x + 1, start_y + 1);
        let sum = left_sum.max(right_sum);
        sum + val
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_len = 2 * self.rows.iter().map(|row| row.len()).max().unwrap();
        for (ind, row) in self.rows.iter().enumerate() {
            let leading_spaces = max_len - 2 * ind;
            let spaces = iter::repeat(" ".to_owned())
                .take(leading_spaces)
                .collect::<Vec<String>>()
                .join("");
            write!(f, "{spaces}")?;
            for val in row.iter() {
                write!(f, " {val:02} ")?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn main() {
    let triangle = Triangle::load();
    println!("{triangle}");
    println!("{}", triangle.max_sum(0, 0));
}

use std::fs::read_to_string;

const TRIANGLE_FILE: &str = "triangle.txt";

type TriangleRow = Vec<usize>;

#[derive(Debug, Clone)]
struct Triangle {
    rows: Vec<TriangleRow>,
}

fn load_triangle() -> Triangle {
    let contents = read_to_string(TRIANGLE_FILE).unwrap();
    let mut triangle = Triangle { rows: vec![] };
    for line in contents.lines() {
        let line_nums: TriangleRow = line
            .split(' ')
            .map(|num| num.parse::<usize>().unwrap())
            .collect();
        triangle.rows.push(line_nums);
    }
    triangle
}

fn max_from(triangle: Triangle, start: usize) -> usize {
    if triangle.rows.is_empty() {
        return 0;
    }
    if start >= triangle.rows.last().unwrap().len() {
        return 0;
    }
    let mut triangle = triangle;
    let bottom_num = triangle.rows.pop().unwrap()[start];
    if triangle.rows.is_empty() {
        return bottom_num;
    }
    let mut sum_left = if start > 0 {
        max_from(triangle.clone(), start - 1)
    } else {
        0
    };
    sum_left += bottom_num;

    let mut sum_right = if start <= triangle.rows[0].len() - 1 {
        max_from(triangle, start)
    } else {
        0
    };
    sum_right += bottom_num;
    sum_left.max(sum_right)
}

fn max_sum(triangle: Triangle) -> usize {
    if triangle.rows.is_empty() {
        return 0;
    }
    let bottom = triangle.rows.last().unwrap();
    let mut max_sum = 0;
    for (i, _) in bottom.iter().enumerate() {
        let sum = max_from(triangle.clone(), i);
        max_sum = sum.max(max_sum);
    }
    max_sum
}

fn main() {
    let triangle = load_triangle();
    println!("sum : {}", max_sum(triangle));
}

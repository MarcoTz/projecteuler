use std::{fmt, fs::read_to_string};

const GRID_FILENAME: &str = "grid.txt";
const GRID_WIDTH: usize = 20;
const GRID_HEIGHT: usize = 20;

enum Dir {
    Row,
    Column,
    Diag,
    InvDiag,
}

#[derive(Copy, Clone)]
struct FourNums {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
}

impl FourNums {
    fn prod(self) -> u64 {
        self.a * self.b * self.c * self.d
    }
}

struct Grid([[u64; GRID_WIDTH]; GRID_HEIGHT]);

impl Grid {
    fn load() -> Grid {
        let mut grid = [[0; GRID_WIDTH]; GRID_HEIGHT];
        let contents = read_to_string(GRID_FILENAME).unwrap();
        for (line_num, line) in contents.lines().enumerate() {
            let nums = line
                .split(' ')
                .map(|num| num.trim().parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            for (num_num, num) in nums.iter().enumerate() {
                grid[line_num][num_num] = *num;
            }
        }
        Grid(grid)
    }

    fn four_dir(&self, start_x: usize, start_y: usize, dir: Dir) -> Option<FourNums> {
        let coords = match dir {
            Dir::Row => (
                (start_x, start_y),
                (start_x + 1, start_y),
                (start_x + 2, start_y),
                (start_x + 3, start_y),
            ),
            Dir::Column => (
                (start_x, start_y),
                (start_x, start_y + 1),
                (start_x, start_y + 2),
                (start_x, start_y + 3),
            ),
            Dir::Diag => (
                (start_x, start_y),
                (start_x + 1, start_y + 1),
                (start_x + 2, start_y + 2),
                (start_x + 3, start_y + 3),
            ),
            Dir::InvDiag => {
                if start_y < 3 {
                    return None;
                }
                (
                    (start_x, start_y),
                    (start_x + 1, start_y - 1),
                    (start_x + 2, start_y - 2),
                    (start_x + 3, start_y - 3),
                )
            }
        };
        let a = *self.0.get(coords.0 .0)?.get(coords.0 .1)?;
        let b = *self.0.get(coords.1 .0)?.get(coords.1 .1)?;
        let c = *self.0.get(coords.2 .0)?.get(coords.2 .1)?;
        let d = *self.0.get(coords.3 .0)?.get(coords.3 .1)?;
        Some(FourNums { a, b, c, d })
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.0.iter() {
            for val in row.iter() {
                write!(f, " {val:02} ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Display for FourNums {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{},{}", self.a, self.b, self.c, self.d)
    }
}
fn main() {
    let grid = Grid::load();
    let mut prods = vec![];
    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            prods.push(grid.four_dir(x, y, Dir::Row).map(FourNums::prod));
            prods.push(grid.four_dir(x, y, Dir::Column).map(FourNums::prod));
            prods.push(grid.four_dir(x, y, Dir::Diag).map(FourNums::prod));
            prods.push(grid.four_dir(x, y, Dir::InvDiag).map(FourNums::prod));
        }
    }
    let remaining = prods.into_iter().filter_map(|x| x);
    println!("{}", remaining.max().unwrap())
}

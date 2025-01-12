use core::array;
const GRID_WIDTH: usize = 20;
const GRID_HEIGHT: usize = 20;

type GridRow = [Cell; GRID_WIDTH];
type Coordinate = (usize, usize);

#[derive(Default, Debug)]
struct Cell {
    num_incoming: Option<usize>,
    num_outgoing: Option<usize>,
}

impl Cell {
    pub fn filled(&self) -> bool {
        self.num_incoming.is_some() && self.num_outgoing.is_some()
    }
}

#[derive(Debug)]
struct Grid {
    rows: [GridRow; GRID_HEIGHT],
}

impl Grid {
    pub fn new() -> Grid {
        let mut rows = array::from_fn(|_| array::from_fn(|_| Cell::default()));
        rows[0][0] = Cell {
            num_incoming: Some(0),
            num_outgoing: Some(2),
        };
        Grid { rows }
    }

    pub fn above(&self, i: usize, j: usize) -> Option<&Cell> {
        if j > 0 {
            Some(&self.rows[i][j - 1])
        } else {
            None
        }
    }

    pub fn left(&self, i: usize, j: usize) -> Option<&Cell> {
        if i > 0 {
            Some(&self.rows[i - 1][j])
        } else {
            None
        }
    }

    pub fn get_next(&self) -> Option<(Coordinate, usize, usize)> {
        for i in 0..GRID_WIDTH {
            for j in 0..GRID_HEIGHT {
                if self.rows[i][j].filled() {
                    continue;
                }
                let num_above = if let Some(ab) = self.above(i, j) {
                    if !ab.filled() {
                        continue;
                    }
                    ab.num_outgoing.unwrap()
                } else {
                    0
                };
                let num_left = if let Some(lf) = self.left(i, j) {
                    if !lf.filled() {
                        continue;
                    }
                    lf.num_outgoing.unwrap()
                } else {
                    0
                };
                return Some(((i, j), num_above, num_left));
            }
        }
        None
    }

    pub fn fill(&mut self) {
        while let Some(((i, j), num_above, num_left)) = self.get_next() {
            let cell = &mut self.rows[i][j];
            cell.num_incoming = Some(num_above + num_left - 2);
            cell.num_outgoing = Some(num_above + num_left);
        }
    }
}

impl Grid {}

fn main() {
    let mut grid = Grid::new();
    grid.fill();
    let last = &grid.rows[GRID_WIDTH - 1][GRID_HEIGHT - 1];
    println!("number of paths: {}", last.num_outgoing.unwrap());
}

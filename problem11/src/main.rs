use std::fs::read_to_string;

const GRID_FILENAME: &str = "grid.txt";
const GRID_WIDTH: usize = 20;
const GRID_HEIGHT: usize = 20;

pub type Grid = [[i32; GRID_WIDTH]; GRID_HEIGHT];

fn load_grid() -> Result<Grid, String> {
    let mut grid = [[0; GRID_WIDTH]; GRID_HEIGHT];
    let contents = read_to_string(GRID_FILENAME).map_err(|err| err.to_string())?;
    for (line_num, line) in contents.lines().enumerate() {
        let nums = line
            .split(' ')
            .map(|num| num.trim().parse::<i32>().map_err(|err| err.to_string()))
            .collect::<Result<Vec<i32>, String>>()
            .map_err(|err| err.to_string())?;
        for (num_num, num) in nums.iter().enumerate() {
            grid[line_num][num_num] = *num;
        }
    }
    Ok(grid)
}

fn max_prod_row(grid: &Grid, row_num: usize) -> i32 {
    let mut prod = grid[row_num][0] * grid[row_num][1] * grid[row_num][2] * grid[row_num][3];
    if prod == 0 {
        return 0;
    };
    let mut max_prod = prod;
    for i in 4..GRID_WIDTH {
        if grid[row_num][i] == 0 {
            return max_prod;
        };
        prod *= grid[row_num][i] / grid[row_num][i - 4];
        max_prod = prod.max(max_prod);
    }
    max_prod
}

fn max_prod_col(grid: &Grid, col_num: usize) -> i32 {
    let mut prod = grid[0][col_num] * grid[1][col_num] * grid[2][col_num] * grid[3][col_num];
    if prod == 0 {
        return prod;
    };
    let mut max_prod = prod;
    for i in 4..GRID_HEIGHT {
        if grid[i][col_num] == 0 {
            return max_prod;
        };

        prod *= grid[i][col_num] / grid[i - 4][col_num];
        max_prod = prod.max(max_prod);
    }
    max_prod
}

fn max_prod_diag(grid: &Grid, diag_num: usize) -> i32 {
    if diag_num >= GRID_HEIGHT - 4 {
        return 0;
    }

    let mut prod =
        grid[diag_num][0] * grid[diag_num + 1][1] * grid[diag_num + 2][2] * grid[diag_num + 3][3];
    if prod == 0 {
        return 0;
    }
    let mut max_prod = prod;
    for i in 4..GRID_HEIGHT.min(GRID_WIDTH) {
        if diag_num + i >= GRID_WIDTH - 1 {
            return max_prod;
        }
        if grid[diag_num + i][i] == 0 {
            return max_prod;
        }

        prod *= grid[diag_num + i][i] / grid[diag_num + i - 4][i - 4];
        max_prod = max_prod.max(prod);
    }
    max_prod
}

fn main() -> Result<(), String> {
    let grid = load_grid()?;
    let mut max_prod = 1;
    for i in 0..GRID_WIDTH.min(GRID_HEIGHT) {
        let max_row = max_prod_row(&grid, i);
        let max_col = max_prod_col(&grid, i);
        let max_diag = max_prod_diag(&grid, i);
        max_prod = max_prod.max(max_row).max(max_col).max(max_diag);
    }
    println!("{max_prod}",);
    Ok(())
}

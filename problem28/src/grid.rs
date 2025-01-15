use crossterm::{cursor, queue, style, terminal};
use std::{
    collections::HashMap,
    io,
    io::{Stdout, Write},
};

const CELL_SIZE: i64 = 3;
pub type GridValue = usize;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct GridPosition {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Default)]
pub struct Grid {
    cells: HashMap<GridPosition, GridValue>,
}

impl Grid {
    pub fn add_cell(&mut self, val: GridValue, x: i64, y: i64) {
        self.cells.insert(GridPosition { x, y }, val);
    }

    pub fn diagonal_sum(&self) -> usize {
        let mut sum = 0;
        for (pos, val) in self.cells.iter() {
            if pos.x == pos.y || pos.x == -pos.y {
                sum += *val;
            }
        }
        sum
    }

    pub fn display(&self, stdout: &mut Stdout) -> io::Result<()> {
        queue!(stdout, terminal::Clear(terminal::ClearType::All))?;
        let (height, width) = terminal::size()?;
        let (center_x, center_y) = ((height + 1) / 2, (width + 1) / 2);
        for (pos, val) in self.cells.iter() {
            let x = if pos.x >= 0 {
                center_x + ((CELL_SIZE * pos.x) as u16)
            } else {
                center_x - ((CELL_SIZE * (-pos.x)) as u16)
            };
            let y = if pos.y >= 0 {
                center_y - (((CELL_SIZE - 1) * pos.y) as u16)
            } else {
                center_y + (((CELL_SIZE - 1) * (-pos.y)) as u16)
            };
            queue!(stdout, cursor::MoveTo(x, y), style::Print(val))?;
        }
        queue!(stdout, cursor::MoveTo(height, width), style::Print(" "))?;
        stdout.flush()?;
        Ok(())
    }
}

use super::grid::{Grid, GridPosition, GridValue};

#[derive(Debug)]
pub enum Dir {
    Left,
    Right,
    Up,
    Down,
}

pub struct SpiralWalker {
    pub current_position: GridPosition,
    pub current_val: GridValue,
    pub grid: Grid,
}

impl SpiralWalker {
    pub fn walk(&mut self) {
        self.update_pos();

        self.current_val += 1;
        self.grid.add_cell(
            self.current_val,
            self.current_position.x,
            self.current_position.y,
        );
    }

    fn current_square_size(&self) -> usize {
        let mut next_bigger = (self.current_val as f32).sqrt().floor() as usize;
        while next_bigger % 2 == 0 || next_bigger * next_bigger < self.current_val {
            next_bigger += 1;
        }
        next_bigger
    }

    fn next_dir(&self) -> Dir {
        let size = self.current_square_size();
        if size == 1 {
            return Dir::Right;
        }

        let right_min = Self::square_start_val(size);
        let right_max = right_min + size - 2;
        let bottom_min = right_max;
        let bottom_max = bottom_min + size - 1;
        let left_min = bottom_max;
        let left_max = left_min + size - 1;
        let top_min = left_max;
        let top_max = top_min + size - 1;

        match self.current_val {
            i if i >= right_min && i < right_max => Dir::Down,
            i if i >= bottom_min && i < bottom_max => Dir::Left,
            i if i >= left_min && i < left_max => Dir::Up,
            i if i >= top_min && i <= top_max => Dir::Right,
            i => panic!("{i} does not fit in grid"),
        }
    }

    fn update_pos(&mut self) {
        let (x, y) = (self.current_position.x, self.current_position.y);
        match self.next_dir() {
            Dir::Down => self.current_position = GridPosition { x, y: y - 1 },
            Dir::Left => self.current_position = GridPosition { x: x - 1, y },
            Dir::Up => self.current_position = GridPosition { x, y: y + 1 },
            Dir::Right => self.current_position = GridPosition { x: x + 1, y },
        }
    }

    fn square_start_val(square_size: usize) -> GridValue {
        if square_size == 1 {
            return 1;
        }
        (square_size - 2) * (square_size - 2) + 1
    }
}

impl Default for SpiralWalker {
    fn default() -> SpiralWalker {
        let mut walker = SpiralWalker {
            current_position: GridPosition { x: 0, y: 0 },
            current_val: 1,
            grid: Grid::default(),
        };
        walker.grid.add_cell(1, 0, 0);
        walker
    }
}

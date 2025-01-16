use std::collections::HashMap;

const GRID_WIDTH: u64 = 20;
const GRID_HEIGHT: u64 = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GridPoint {
    x: u64,
    y: u64,
}

impl GridPoint {
    fn paths(self, paths_cache: &mut HashMap<GridPoint, u64>) -> u64 {
        if let Some(paths) = paths_cache.get(&self) {
            return *paths;
        }

        if self.start() {
            return 1;
        }
        let (x, y) = (self.x, self.y);
        let paths_up = if y > 0 {
            let above = GridPoint { x, y: y - 1 };
            above.paths(paths_cache)
        } else {
            0
        };

        let paths_left = if x > 0 {
            let left = GridPoint { x: x - 1, y };
            left.paths(paths_cache)
        } else {
            0
        };

        let paths = paths_up + paths_left;
        paths_cache.insert(self, paths);

        paths
    }

    fn start(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}
fn main() {
    let end = GridPoint {
        x: GRID_WIDTH,
        y: GRID_HEIGHT,
    };
    let paths = end.paths(&mut Default::default());
    println!("{}", paths);
}

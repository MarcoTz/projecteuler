use std::{io /*thread, time*/};
mod grid;
mod spiral;
use spiral::SpiralWalker;

const MAX_NUM: usize = 1002001;

fn main() -> io::Result<()> {
    let mut walker = SpiralWalker::default();
    //let mut stdout = io::stdout();
    //let time = time::Duration::from_millis(100);
    for _ in 1..=MAX_NUM {
        //walker.grid.display(&mut stdout)?;
        walker.walk();
        //thread::sleep(time);
    }
    println!("{}", walker.grid.diagonal_sum());
    Ok(())
}

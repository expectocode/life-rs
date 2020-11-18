extern crate grid;
extern crate structopt;

use grid::Grid;
use std::{thread, time};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "life-rs")]
struct Opt {
    #[structopt(
        short = "c",
        long = "columns",
        default_value = "80",
        help = "The number of columns that will be printed to the terminal (same as the number of columns in the game grid)"
    )]
    columns: usize,

    #[structopt(
        short = "r",
        long = "rows",
        default_value = "20",
        help = "The number of rows that will be printed to the terminal. Note that the number of rows in the game grid is twice this amount, as each printed row is two game rows."
    )]
    rows: usize,

    #[structopt(
        long = "iter-count",
        help = "BENCHMARKING TOOL: num iterations of game"
    )]
    iter_count: Option<usize>,

    #[structopt(
        short = "d",
        long = "delay",
        default_value = "60",
        help = "Duration to wait between each generation (milliseconds)"
    )]
    delay: u64,

    #[structopt(
        short = "a",
        long = "alive-chance",
        default_value = "0.2",
        help = "Chance for cells to be alive in the first generation"
    )]
    alive_chance: f64,
}

/// Count the number of neighbouring cells that are alive, not including itself
fn alive_neighbours(grid: &Grid, x: usize, y: usize) -> usize {
    let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    offsets
        .iter()
        .filter(|(x_o, y_o)| grid[(x_o + x as isize, y_o + y as isize)])
        .count()
}

/// Compute the next generation from `grid` and place the result in `next`
fn generation(grid: &Grid, next: &mut Grid) {
    for (x, y, &alive) in grid.iter() {
        next[(x, y)] = match alive_neighbours(&grid, x, y) {
            2 => alive,
            3 => true,
            _ => false,
        };
    }
}

fn draw(grid: &Grid) {
    println!("{}", grid.stringify());
    println!("\x1b[0;0H");
}

fn main() {
    let opt = Opt::from_args();

    let (width, height) = (opt.columns, opt.rows * 2);

    let mut grid = Grid::new(width, height);
    let mut next_grid = Grid::new(width, height);
    grid.randomise(opt.alive_chance);

    println!("\x1b[2J\x1b[H");

    let iter_count = match opt.iter_count {
        Some(a) => a,
        None => usize::max_value(),
    };

    for _ in 0..iter_count {
        draw(&grid);
        generation(&grid, &mut next_grid);
        std::mem::swap(&mut next_grid, &mut grid);
        thread::sleep(time::Duration::from_millis(opt.delay));
    }
}

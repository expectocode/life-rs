extern crate rand;
use std::ops::{Index, IndexMut};
use std::vec::Vec;

use rand::{thread_rng, Rng};

/// A representation of the grid of the Game of Life.
/// Indexing wraps around the grid edges, effectively making it a torus
pub struct Grid {
    cells: Vec<bool>,
    width: usize,
    height: usize,
}

impl Grid {
    /// Create a new empty (false-initialised) Grid. Width and height must be positive.
    pub fn new(width: usize, height: usize) -> Grid {
        if width == 0 || height == 0 {
            panic!("cannot create zero size grid");
        }

        Grid {
            cells: vec![false; width * height],
            width,
            height,
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    /// Set every cell's state randomly.
    /// `live_chance` is the probability that a given cell will be `true`.
    pub fn randomise(&mut self, live_chance: f64) {
        let mut rng = thread_rng();
        for y in 0..self.height {
            for x in 0..self.width {
                self[(x, y)] = rng.gen_bool(live_chance);
            }
        }
    }

    /// Set the cell at each (x, y) coordinate to `true`.
    ///
    /// `x_offset` and `y_offset` are added to each x and y.
    pub fn set_cells(&mut self, coords: &[(usize, usize)], x_offset: usize, y_offset: usize) {
        coords
            .iter()
            .map(|(x, y)| (x + x_offset, y + y_offset))
            .for_each(|coord| self[coord] = true);
    }

    /// Get an ASCII-art representation of the whole grid, with lines to indicate the edges.
    ///
    /// Note: the representation uses half the number of rows since two grid rows are rendered in
    /// each printed row.
    pub fn stringify(&self) -> String {
        let mut lines = Vec::with_capacity((self.height / 2) + 2);
        lines.push("+".to_owned() + &"-".repeat(self.width) + "+");
        self.cells
            .chunks_exact(self.width * 2)
            .map(|cells| stringify_row_pair(&cells[..self.width], &cells[self.width..]))
            .for_each(|s| lines.push(s));
        lines.push("+".to_owned() + &"-".repeat(self.width) + "+");

        lines.join("\n")
    }

    /// Get an iterator over the rows of the grid.
    pub fn iter_rows(&self) -> impl Iterator<Item = &[bool]> {
        self.cells.chunks_exact(self.width)
    }

    /// Get an iterator over the cells of the grid.
    /// Yields a tuple (x, y, value)
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &bool)> {
        let width = self.width;
        self.cells
            .iter()
            .enumerate()
            .map(move |(i, alive)| (i % width, i / width, alive))
    }
}

/// Get an ASCII-art representation of two rows, intended for use with `Grid.stringify()`.
fn stringify_row_pair(up_row: &[bool], down_row: &[bool]) -> String {
    let mut s = String::with_capacity(up_row.len() + 2);
    s.push('|');
    up_row
        .iter()
        .zip(down_row.iter())
        .map(|pair| match pair {
            (false, true) => '▄',
            (true, false) => '▀',
            (true, true) => '█',
            _ => ' ',
        })
        .for_each(|c| s.push(c));
    s.push('|');

    s
}

/// Euclidean modulo
/// As long as n is positive, the result will always be positive
fn modulo(a: isize, n: isize) -> usize {
    if a < 0 {
        ((a % n + n) % n) as usize
    } else {
        (a % n) as usize
    }
}

/// Index directly into the cells of the grid.
impl Index<(usize, usize)> for Grid {
    type Output = bool;

    fn index(&self, xy: (usize, usize)) -> &Self::Output {
        let (x, y) = xy;
        let (x, y) = (x % self.width, y % self.height);
        &self.cells[y * self.width + x]
    }
}

/// Index directly into the rows of the grid.
impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, xy: (usize, usize)) -> &mut Self::Output {
        let (x, y) = xy;
        let (x, y) = (x % self.width, y % self.height);
        &mut self.cells[y * self.width + x]
    }
}

/// Index directly into the cells of the grid with isize coordinates.
impl Index<(isize, isize)> for Grid {
    type Output = bool;

    fn index(&self, xy: (isize, isize)) -> &Self::Output {
        let (x, y) = xy;
        let (x, y) = (modulo(x, self.width as isize), modulo(y, self.height as isize));
        &self.cells[y * self.width + x]
    }
}

/// Index directly into the rows of the grid with isize coordinates.
impl IndexMut<(isize, isize)> for Grid {
    fn index_mut(&mut self, xy: (isize, isize)) -> &mut Self::Output {
        let (x, y) = xy;
        let (x, y) = (modulo(x, self.width as isize), modulo(y, self.height as isize));
        &mut self.cells[y * self.width + x]
    }
}

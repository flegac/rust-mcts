use std::iter::{Filter, FromIterator, Map};
use std::ops::Range;

use bit_set::BitSet;
use itertools::{iproduct, Itertools, Product};

pub type GoCell = usize;

pub struct Goban {
    pub size: usize,
    pub cells: BitSet,
}

impl Goban {
    pub fn new(size: usize) -> Self {
        let mut cells = BitSet::new();
        cells.extend((0..(size * size)));
        Goban {
            size,
            cells,
        }
    }

    pub fn cell(&self, x: usize, y: usize) -> GoCell {
        x * self.size + y
    }

    pub fn xy(&self, cell: GoCell) -> (usize, usize) {
        let x = cell as usize / self.size;
        let y = cell as usize % self.size;
        (x, y)
    }

    pub fn adjacents(&self, cell: GoCell) -> BitSet {
        let (x0, y0) = self.xy(cell);

        let convert = |x: usize, y: usize| self.cell(x, y);
        let size = self.size;


        BitSet::from_iter(
            (iproduct![0..3,0..3])
                .into_iter()
                .filter(|(dx, dy)| *dx == 1 || *dy == 1)
                .filter(|(dx, dy)| *dx != 1 || *dy != 1)
                .map(|(dx, dy)| (x0 + dx, y0 + dy))
                .filter(|(x, y)| *x > 0 && *x <= size)
                .filter(|(x, y)| *y > 0 && *y <= size)
                .map(|(x, y)| (x - 1, y - 1))
                .map(|(x, y)| convert(x, y))
        )
    }
}

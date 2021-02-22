use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::ops::Deref;
use std::rc::Rc;

use bit_set::BitSet;
use itertools::{iproduct, Itertools};

pub type GoCell = usize;


#[derive(Hash, Eq, PartialEq)]
pub struct Grid {
    pub size: usize,
    pub cells: BitSet,
    pub edges: Vec<BitSet>,
}

impl Grid {
    pub fn new(size: usize) -> Self {
        Grid {
            size,
            cells: BitSet::from_iter(0..(size * size)),
            edges: (0..(size * size))
                .map(|c| Grid::get_adjacents(size, c))
                .collect_vec(),
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

    pub fn flood<F>(&self, cell: GoCell, test: &F) -> BitSet
        where F: Fn(GoCell) -> bool {
        let mut visited = BitSet::new();
        let mut to_visit = BitSet::new();
        to_visit.insert(cell);
        visited.insert(cell);

        while !to_visit.is_empty() {
            let mut connected = BitSet::new();
            for c in to_visit.iter() {
                self.edges[c].iter()
                    .filter(|&c| test(c))
                    .filter(|&a| !visited.contains(a))
                    .for_each(|c| {
                        connected.insert(c);
                    });
            }


            visited.union_with(&connected);
            to_visit = connected;
        }
        visited
    }

    fn get_adjacents(size: usize, cell: GoCell) -> BitSet {
        let size = size as i32;
        let limit = (size * size) as i32;
        let c = cell as i32;

        let same_line1 = |x: i32| (x % size - c % size).abs() <= 1;
        let in_board = |x: i32| x >= 0 && x < limit;

        BitSet::from_iter([c - 1, c + 1, c - size, c + size].iter()
            .filter(|&x| in_board(*x))
            .filter(|&x| same_line1(*x))
            .map(|&x| x as usize))
    }
}

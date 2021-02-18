use std::fmt;
use std::fmt::Formatter;

use bit_set::BitSet;

use board::goban::{Goban, GoCell};
use stones::stone::Stone;

#[derive(Hash, Eq, PartialEq)]
pub(crate) struct GoGroup {
    pub(crate) stone: Stone,
    pub(crate) liberties: BitSet,
    pub(crate) cells: BitSet,
}

impl GoGroup {
    pub(crate) fn new(stone: Stone, cells: BitSet) -> GoGroup {
        GoGroup {
            stone,
            cells,
            liberties: BitSet::new(),
        }
    }
    pub(crate) fn size(&self) -> usize {
        self.cells.len()
    }


    pub(crate) fn add_group(&mut self, other: &GoGroup) {
        assert!(self.stone == other.stone);
        self.cells.union_with(&other.cells);
    }

    pub(crate) fn remove_group(&mut self, other: &GoGroup) {
        self.cells.difference_with(&other.cells);
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    pub(crate) fn set_stone(&mut self, stone: Stone) {
        self.stone = stone;
    }
}

impl fmt::Display for GoGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        res.push_str(&self.stone.to_string());
        res.push_str("[");
        for c in self.cells.iter() {
            res.push_str(format!("{} ", c).as_str());
        }
        res.push_str("]");

        write!(f, "{}", res)
    }
}

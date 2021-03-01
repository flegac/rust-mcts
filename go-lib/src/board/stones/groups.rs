use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::ops::Deref;

use bit_set::BitSet;

use board::go_state::GoState;
use board::grid::{GoCell, Grid};
use board::stones::stone::Stone;
use graph_lib::algo::flood::Flood;
use graph_lib::graph::GFlood;
use graph_lib::topology::Topology;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct GoGroup {
    pub(crate) id: usize,
    pub(crate) stone: Stone,
    pub(crate) liberties: usize,
    pub(crate) cells: BitSet,
}

impl Clone for GoGroup {
    fn clone(&self) -> Self {
        GoGroup {
            id: self.id,
            stone: self.stone,
            liberties: self.liberties,
            cells: self.cells.clone(),
        }
    }
}

impl GoGroup {
    pub fn from_goban(goban: &Grid) -> GoGroup {
        GoGroup {
            id: 0,
            stone: Stone::None,
            cells: goban.vertices().clone(),
            liberties: 0,
        }
    }

    pub fn new() -> GoGroup {
        GoGroup {
            id: 0,
            stone: Stone::None,
            cells: BitSet::new(),
            liberties: 0,
        }
    }

    pub fn from_cells(stone: Stone, cells: &[GoCell]) -> GoGroup {
        let liberties = match cells.len() {
            1 => 4,
            _ => 0
        };
        GoGroup {
            id: 0,
            stone,
            cells: BitSet::from_iter(cells.deref().iter().map(|x| x.clone())),
            liberties,
        }
    }

    pub(crate) fn stones(&self) -> usize {
        self.cells.len()
    }


    pub(crate) fn add_cells(&mut self, cells: &BitSet) {
        self.cells.union_with(cells);
    }

    pub(crate) fn add_group(&mut self, other: &GoGroup) {
        assert_eq!(self.stone, other.stone);
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

    pub fn is_dead(&self) -> bool {
        self.liberties == 0
    }


    pub fn split(&mut self, board: &GoState) -> Vec<GoGroup> {
        let mut res = vec![];
        while !self.is_empty() {
            res.push(self.next_split(board));
        }
        res
    }

    fn next_split(&mut self, board: &GoState) -> GoGroup {
        let to_visit = self.cells.clone();
        let cell = to_visit.iter().next().unwrap();

        let test = |x| self.cells.contains(x);
        let res = GoGroup {
            id: 0,
            stone: self.stone,
            cells: GFlood::new().flood(board, cell, &test),
            liberties: 0,
        };
        self.remove_group(&res);
        res
    }
}

impl Hash for GoGroup {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::convert::TryFrom;
    use std::hash::{Hash, Hasher};

    use board::go_state::GoState;
    use board::grid::Grid;
    use board::stones::groups::GoGroup;
    use board::stones::stone::Stone;

    #[test]
    fn group_hash() {
        let g1 = GoGroup::from_cells(Stone::None, &[33, 36, 44]);
        let g1_bis = GoGroup::from_cells(Stone::None, &[33, 36, 44]);
        let g2 = GoGroup::from_cells(Stone::None, &[33, 128, 3000]);
        let g3 = GoGroup::from_cells(Stone::None, &[33, 36, 10]);

        assert_eq!(g1, g1_bis);
        assert_ne!(g1, g2);
        assert_ne!(g1, g3);

        assert_eq!(hash(&g1), hash(&g2));
        assert_ne!(hash(&g1), hash(&g3));
    }

    fn hash(x: &GoGroup) -> u64 {
        let mut hasher = DefaultHasher::new();
        x.hash(&mut hasher);
        let h = hasher.finish();
        h
    }
}

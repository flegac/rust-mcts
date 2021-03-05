use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::ops::Deref;

use bit_set::BitSet;

use board::grid::{GoCell, Grid};
use board::stones::stone::Stone;
use graph_lib::algo::flood::Flood;
use graph_lib::graph::GFlood;
use graph_lib::topology::Topology;

use crate::board::go_state::GoState;
use board::stats::board_stats::BoardStats;
use board::group_access::GroupAccess;

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



    pub(crate) fn split_remove(&mut self, cells: BitSet) -> GoGroup {
        let res = GoGroup {
            id: 0,
            stone: self.stone,
            cells,
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

    use board::grid::Grid;
    use board::stones::group::GoGroup;
    use board::stones::stone::Stone;

    use crate::board::go_state::GoState;

    #[test]
    fn group_clone() {
        let mut g1 = GoGroup::from_cells(Stone::None, &[33, 36, 44]);
        let mut g2 = GoGroup::from_cells(Stone::None, &[33, 36, 44]);
        let mut g3 = g1.clone();

        g1.id = 1;
        g2.id = 1;
        assert_eq!(g1, g2);

        g2.id = 2;
        assert_ne!(g1, g2);

        assert_ne!(g1, g3);
        g3.id = g1.id;
        assert_eq!(g1, g3);
        println!("{}", g1);
        println!("{}", g3);
    }


    #[test]
    fn group_eq() {
        let g1 = GoGroup::from_cells(Stone::None, &[33, 36, 44]);
        let g2 = GoGroup::from_cells(Stone::None, &[33, 36, 44]);
        let g3 = GoGroup::from_cells(Stone::None, &[33, 36, 10]);

        assert_eq!(g1, g2);
        assert_ne!(g1, g3);
    }

    fn hash(x: &GoGroup) -> u64 {
        let mut hasher = DefaultHasher::new();
        x.hash(&mut hasher);
        let h = hasher.finish();
        h
    }
}

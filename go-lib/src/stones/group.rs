use std::fmt;
use std::fmt::Formatter;
use std::iter::{FromIterator, once};

use bit_set::BitSet;

use board::goboard::GoBoard;
use board::grid::GoCell;
use graph_lib::graph::Graph;
use stones::stone::Stone;

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct GoGroup {
    pub(crate) stone: Stone,
    pub(crate) liberties: usize,
    pub(crate) cells: BitSet,
}

impl GoGroup {
    pub fn from_cell(stone: Stone, cell: GoCell) -> GoGroup {
        GoGroup {
            stone,
            cells: BitSet::from_iter(once(cell)),
            liberties: 4,
        }
    }

    pub(crate) fn stones(&self) -> usize {
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

    pub fn is_dead(&self) -> bool {
        self.liberties == 0
    }

    pub fn adjacent_cells(&self, board: &GoBoard) -> BitSet {
        let mut adjacents = BitSet::new();
        for c in self.cells.iter() {
            adjacents.union_with(&board.edges(c));
        }
        adjacents.difference_with(&self.cells);
        adjacents
    }


    pub fn update_liberties(&mut self, board: &GoBoard) {
        let adjacents = self.adjacent_cells(board);
        let mut liberties = BitSet::new();
        for x in adjacents.iter()
            .filter(|c| board.stone_at(c) == Stone::None) {
            liberties.insert(x);
        }
        self.liberties = liberties.len();
    }


    pub fn split<G: Graph>(&mut self, graph: &G) -> Vec<GoGroup> {
        let mut res = vec![];
        while !self.is_empty() {
            res.push(self.next_split(graph));
        }
        res
    }

    fn next_split<G: Graph>(&mut self, grid: &G) -> GoGroup {
        let to_visit = &self.cells;
        let test = |c: GoCell| to_visit.contains(c);
        let cell = to_visit.iter().next().unwrap();
        let res = GoGroup {
            stone: self.stone,
            cells: grid.flood(cell, &test),
            liberties: 0,
        };
        self.remove_group(&res);
        res
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

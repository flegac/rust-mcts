use std::fmt;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::iter::{FromIterator, once};

use bit_set::BitSet;

use board::go::Go;
use board::goboard::GoBoard;
use board::grid::{GoCell, Grid};
use go_display::GoDisplay;
use graph_lib::algo::flood::Flood;
use graph_lib::topology::{SubGraph, Topology};
use stones::stone::Stone;

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct GoGroup {
    pub(crate) stone: Stone,
    pub(crate) liberties: usize,
    pub(crate) cells: BitSet,
}

impl GoGroup {
    pub fn from_goban(goban: &Grid) -> GoGroup {
        GoGroup {
            stone: Stone::None,
            cells: goban.vertices().clone(),
            liberties: 0,
        }
    }

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

    pub fn update_liberties(&mut self, board: &GoBoard) {
        let mut adjacents = Go::adjacent_cells(board, &self.cells);
        adjacents.intersect_with(&board.empty_cells.cells);
        self.liberties = adjacents.len();
    }


    pub fn split(&mut self, board: &GoBoard) -> Vec<GoGroup> {
        let mut res = vec![];
        while !self.is_empty() {
            res.push(self.next_split(board));
        }
        res
    }

    fn next_split(&mut self, board: &GoBoard) -> GoGroup {
        let to_visit = self.cells.clone();
        let cell = to_visit.iter().next().unwrap();

        let test = |x| self.cells.contains(x);
        let res = GoGroup {
            stone: self.stone,
            cells: board.flood.borrow_mut().flood(board, cell, &test),
            liberties: 0,
        };
        log::trace!("found split: {}", GoDisplay::group(board, &res));
        self.remove_group(&res);
        res
    }
}

impl Hash for GoGroup {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // let stone = self.borrow().stone;
        let min = self.cells.iter().min().unwrap();
        // let max = self.cells.iter().max().unwrap();
        // let x = format!("{}:{}-{}", stone, min, max);
        // x.hash(state)
        min.hash(state)
    }
}

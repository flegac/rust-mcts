use core::fmt;
use std::collections::HashSet;
use std::convert::TryInto;
use std::iter::Map;
use std::ops::Range;
use std::rc::Rc;

use itertools::{iproduct, Itertools, Product};

use mcts_lib::mymcts::MyMcts;

use crate::action::GoAction;
use crate::constants::GOBAN_SIZE;
use crate::stone::Stone;
use crate::stone_group::StoneGroup;

pub type GoCell = usize;

pub(crate) struct GoBoard<> {
    board: [Option<Stone>; GOBAN_SIZE * GOBAN_SIZE],
    groups: HashSet<StoneGroup>,
}


impl GoBoard {
    pub(crate) fn new() -> Self {
        GoBoard {
            board: [None; GOBAN_SIZE * GOBAN_SIZE],
            groups: HashSet::new(),
        }
    }

    pub(crate) fn lines(&self) -> Vec<Vec<usize>> {
        (0..GOBAN_SIZE)
            .map(|y| (0..GOBAN_SIZE)
                .map(|x| self.cell(x, y))
                .collect_vec())
            .collect_vec()
    }

    pub(crate) fn cells(&self) -> Vec<usize> {
        let data = (iproduct![0..GOBAN_SIZE, 0..GOBAN_SIZE])
            .into_iter()
            .map(|(x, y)| self.cell(x, y)).collect_vec();
        data
    }

    pub(crate) fn cell(&self, x: usize, y: usize) -> GoCell {
        x * GOBAN_SIZE + y
    }

    pub(crate) fn xy(&self, cell: GoCell) -> (usize, usize) {
        let x = cell as usize / GOBAN_SIZE;
        let y = cell as usize % GOBAN_SIZE;
        (x, y)
    }

    pub(crate) fn update(&mut self, cell: GoCell, value: Option<Stone>) {
        self.board[cell] = value;
    }


    pub(crate) fn count_stones(&self, stone: Option<Stone>) -> usize {
        // self.groups.into_iter()
        //     .filter(|g| g.stone == stone)
        //     .map(|g| g.cells.len())
        //     .sum()
        self.board.iter()
            .filter(|&&s| s == stone)
            .count()
    }

    pub(crate) fn actions(&self) -> Vec<GoAction> {
        // self.groups.into_iter()
        //     .filter(|g| g.stone == None)
        //     .flat_map(|g| g.cells.iter())
        //     .map(|c| GoAction::play_at(c))
        //     .collect_vec()
        self.cells()
            .iter()
            .filter(|&&c| self.board[c].is_none())
            .map(|&c| GoAction::play_at(c))
            .collect_vec()
    }
}


impl fmt::Display for GoBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::new();

        for x in 0..GOBAN_SIZE {
            for y in 0..GOBAN_SIZE {
                match self.board[self.cell(x, y)] {
                    None => {
                        res.push_str(".");
                    }
                    Some(s) => {
                        res.push_str(&s.to_string());
                    }
                };
                res.push_str(" ");
            }
            res.push_str("\n");
        }
        write!(f, "{}", res)
    }
}

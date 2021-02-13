use core::fmt;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::iter::Map;
use std::ops::Range;
use std::rc::Rc;

use itertools::{iproduct, Itertools, Product};

use mcts_lib::mymcts::MyMcts;
use stones::group::GoGroupRc;
use stones::stone::Stone;

use crate::action::GoAction;
use crate::constants::GOBAN_SIZE;

pub type GoCell = usize;


pub(crate) struct GoBoard<> {
    board: HashMap<GoCell, GoGroupRc>,
}


impl GoBoard {
    pub(crate) fn new() -> Self {
        let mut board = GoBoard {
            board: HashMap::new(),
        };
        board.reset();
        board
    }

    pub(crate) fn reset(&mut self) {
        self.on_board(
            GoGroupRc::new(None)
                .with_cells(self.cells().as_slice()));
    }

    pub(crate) fn on_board(&mut self, stones: GoGroupRc) {
        for c in stones.borrow().cells.iter() {
            self.board.insert(c, stones.clone());
        }
        // self.groups.insert(stones.clone());
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
        // removing cells from old group
        let mut old = self.board.get(&cell);
        old.map(|rc| rc.borrow_mut().cells.remove(cell));
        old.map(|rc| println!("{:?}", rc.borrow().cells));
        //TODO: check if old group connectivity & split it if needed


        // adding stone to new group
        // TODO: find adjacent groups (on adjacent cells) & fusion them together if appropriate
        let cells = vec![cell];

        // // updating cells with new group
        let stones = GoGroupRc::new(value)
            .with_cells(cells.as_slice());
        for c in stones.borrow().cells.iter() {
            self.board.insert(c, stones.clone());
        }
    }


    pub(crate) fn count_stones(&self, stone: Option<Stone>) -> usize {
        // self.groups.into_iter()
        //     .filter(|g| g.stone == stone)
        //     .map(|g| g.cells.len())
        //     .sum()
        self.board.iter()
            .filter(|(c, g)| g.borrow().stone == stone)
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
            .filter(|&c| self.board.get(c).is_none() || self.board.get(c).unwrap().borrow().stone.is_none())
            .map(|&c| GoAction::play_at(c))
            .collect_vec()
    }
}


impl fmt::Display for GoBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::new();

        for x in 0..GOBAN_SIZE {
            for y in 0..GOBAN_SIZE {
                match self.board.get(&self.cell(x, y)) {
                    None => {
                        res.push_str(".");
                    }
                    Some(g) => {
                        match g.borrow().stone {
                            None => {
                                res.push_str(".");
                            }
                            Some(s) => {
                                res.push_str(&s.to_string());
                            }
                        };
                    }
                }


                res.push_str(" ");
            }
            res.push_str("\n");
        }
        write!(f, "{}", res)
    }
}

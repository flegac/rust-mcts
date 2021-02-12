use core::fmt;
use std::collections::HashSet;

use itertools::{iproduct, Itertools};

use crate::action::GoAction;
use crate::cell::GoCell;
use crate::constants::GOBAN_SIZE;
use crate::stone::Stone;
use crate::stone_group::StoneGroup;

pub(crate) struct GoBoard {
    board: [[Option<Stone>; GOBAN_SIZE]; GOBAN_SIZE],
    groups: HashSet<StoneGroup>,
}


impl GoBoard {
    pub(crate) fn new() -> Self {
        GoBoard {
            board: [[None; GOBAN_SIZE]; GOBAN_SIZE],
            groups: HashSet::new(),
        }
    }

    pub(crate) fn update(&mut self, cell: GoCell, value: Option<Stone>) {
        self.board[cell.x][cell.y] = value;
    }


    pub(crate) fn count_stones(&self, stone: Stone) -> usize {
        self.board.iter()
            .flat_map(|line| line.iter())
            .filter(|&&s| s.is_some())
            .map(|&s| s.unwrap())
            .filter(|&s| s == stone)
            .count()
    }

    pub(crate) fn actions(&self) -> Vec<GoAction> {
        let res = (iproduct![0..GOBAN_SIZE, 0..GOBAN_SIZE])
            .filter(|&(x, y)| self.board[x][y] == None)
            .map(|(x, y)| GoAction::play_at(x, y))
            .collect_vec();
        res
    }
}

impl fmt::Display for GoBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::new();

        for x in 0..GOBAN_SIZE {
            for y in 0..GOBAN_SIZE {
                match self.board[x][y] {
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

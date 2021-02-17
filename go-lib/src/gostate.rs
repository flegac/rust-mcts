use core::fmt;
use std::cmp::Ordering;

use itertools::Itertools;

use board::goban::Goban;
use board::goboard::GoBoard;
use mcts_lib::state::{GameResult, State};
use stones::stone::Stone;

use crate::action::GoAction;
use crate::constants::GOBAN_SIZE;

pub struct GoState {
    board: GoBoard,
    stone: Stone,
    pub history: Vec<GoAction>,
}

impl GoState {}


impl State<GoAction> for GoState {
    fn initial() -> GoState {
        GoState {
            board: GoBoard::new(Goban::new(GOBAN_SIZE)),
            stone: Stone::Black,
            history: vec![],
        }
    }

    fn result(&self) -> Option<GameResult> {
        let size = self.board.goban.size;

        let black_stats = &self.board.stats.black;
        let white_stats = &self.board.stats.white;
        let blacks = black_stats.captured + black_stats.territory;
        let whites = white_stats.captured + white_stats.territory;

        match self.board.stats.none.stones.cmp(&10) {
            Ordering::Less => Some(match blacks.cmp(&whites) {
                Ordering::Less => GameResult::Defeat,
                Ordering::Equal => GameResult::Draw,
                Ordering::Greater => GameResult::Victory
            }),
            _ => None
        }
    }


    fn actions(&self) -> Vec<GoAction> {
        self.board.goban.cells
            .iter()
            .filter(|c| self.board.group_at(c).borrow().stone == Stone::None)
            .map(|c| GoAction::at(c))
            .collect_vec()
    }

    fn next(&mut self, action: &GoAction) {
        action.cell.map(|cell| self.board.play_at(cell, self.stone));

        self.stone = self.stone.switch();
        self.history.push(action.clone());
    }
}

impl fmt::Display for GoState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut history = String::new();
        for a in self.history.iter() {
            history.push_str(format!("{} ", a).as_str());
        }

        write!(f, "{}", format!("side: {}\n{}\nhistory({}): {}\n",
                                self.stone,
                                self.board,
                                self.history.len(),
                                history
        ).as_str())
    }
}
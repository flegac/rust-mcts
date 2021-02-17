use core::fmt;

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

        let blacks = self.board.stats.black.stones;
        let whites = self.board.stats.white.stones;
        println!("{}", self);
        if 10 * (whites + blacks) > 9 * size * size {
            Some(GameResult::Victory)
        } else {
            None
        }
    }


    fn actions(&self) -> Vec<GoAction> {
        self.board.goban.cells
            .iter()
            .filter(|c| self.board.group_at(c).borrow().stone == Stone::None)
            .map(|c| GoAction::play_at(c))
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
        let mut res = String::new();
        res.push_str("side: ");
        res.push_str(&self.stone.to_string());
        res.push_str("\n");
        res.push_str(&self.board.to_string());
        res.push_str("\n");

        //history
        res.push_str("history(");
        res.push_str(&self.history.len().to_string());
        res.push_str("): ");
        for a in self.history.iter() {
            res.push_str(&a.to_string());
            res.push_str(" ");
        }
        res.push_str("\n");

        write!(f, "{}", res)
    }
}
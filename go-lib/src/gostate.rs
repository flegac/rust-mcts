use core::fmt;

use mcts_lib::state::{GameResult, State};
use stones::stone::Stone;

use crate::action::GoAction;
use crate::board::GoBoard;
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
            board: GoBoard::new(),
            stone: Stone::Black,
            history: vec![],
        }
    }

    fn result(&self) -> Option<GameResult> {
        let blacks = self.board.count_stones(Some(Stone::Black));
        let whites = self.board.count_stones(Some(Stone::White));
        if 4 * (whites + blacks) > 3 * GOBAN_SIZE * GOBAN_SIZE {
            Some(GameResult::Victory)
        } else {
            None
        }
    }


    fn actions(&self) -> Vec<GoAction> {
        self.board.actions()
    }

    fn next(&mut self, action: &GoAction) {
        action.cell.map(|cell| self.board.update(cell, Some(self.stone)));

        self.stone = self.stone.switch();
        self.history.push(action.clone());
    }

    fn prev(&mut self) {
        match self.history.pop() {
            None => {}
            Some(action) => {
                action.cell.map(|cell| self.board.update(cell, None));
                self.stone = self.stone.switch();
            }
        }
    }
}

impl fmt::Display for GoState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "side: {}\n{}", self.stone, self.board)
    }
}
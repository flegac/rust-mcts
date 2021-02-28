use core::fmt;
use std::cmp::Ordering;

use itertools::Itertools;

use board::goboard::GoBoard;
use board::grid::Grid;
use board::stats::board_stats::FullStats;
use display::display::GoDisplay;
use display::goshow::GoShow;
use game::gogame::Sequence;
use mcts_lib::state::{GameResult, State};

use crate::action::GoAction;

pub struct GoState {
    pub board: GoBoard,
    pub history: Vec<GoAction>,
}

impl GoState {
    pub fn new(size: usize) -> GoState {
        GoState {
            board: GoBoard::new(Grid::new(size)),
            history: vec![],
        }
    }
}


impl State<GoAction> for GoState {
    fn reset(&mut self) {
        self.history.clear();
        self.board.reset();
    }

    fn result(&self) -> Option<GameResult> {
        if self.board.end_game() {
            let player = self.board.score(self.board.stone).score();
            let opponent = self.board.score(self.board.stone.switch()).score();
            let res = match player.cmp(&opponent) {
                Ordering::Less => GameResult::Lose,
                Ordering::Equal => GameResult::Draw,
                Ordering::Greater => GameResult::Win
            };
            Some(res)
        } else {
            None
        }
    }

    fn actions(&self) -> Vec<GoAction> {
        self.board.actions()
    }

    fn apply(&mut self, action: GoAction) {
        self.board.play(action);
        self.history.push(action.clone());
    }
}

impl fmt::Display for GoState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\nhistory({}):\n{}\n",
               GoDisplay::board(&self.board).to_string(),
               self.board.stats.round,
               Sequence::build(self.history.as_slice()))
    }
}

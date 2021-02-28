use core::fmt;
use std::cmp::Ordering;

use board::goboard::GoBoard;
use board::grid::Grid;
use game::gogame::Sequence;
use mcts_lib::state::{GameResult, State};

use crate::action::GoAction;
use itertools::Itertools;
use display::display::GoDisplay;
use display::goshow::GoShow;

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
            let player = self.board.score(self.board.stone);
            let opponent = self.board.score(self.board.stone.switch());
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
        let mut actions = self.board.empty_cells.cells.iter()
            .map(|c| self.board.goban.xy(c))
            .map(|(x, y)| GoAction::Cell(x, y))
            .collect_vec();
        actions.push(GoAction::Pass);
        actions
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

use core::fmt;
use std::cmp::Ordering;

use itertools::Itertools;

use board::goboard::GoBoard;
use board::grid::Grid;
use board::stats_board::BoardStats;
use board::stats_color::ColorStats;
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
            board: GoBoard::new(Grid::new(GOBAN_SIZE)),
            stone: Stone::Black,
            history: vec![],
        }
    }

    fn result(&self) -> Option<GameResult> {
        if self.history.len() == 150 || self.actions().is_empty() {
            let blacks = &self.board.stats.black.score(&self.board);
            let whites = &self.board.stats.white.score(&self.board);
            let res = match blacks.cmp(&whites) {
                Ordering::Less => GameResult::Defeat,
                Ordering::Equal => GameResult::Draw,
                Ordering::Greater => GameResult::Victory
            };
            Some(res)
        } else {
            None
        }
    }


    fn actions(&self) -> Vec<GoAction> {
        self.board.goban.cells
            .iter()
            .filter(|c| self.board.stone_at(c) == Stone::None)
            .map(|c| self.board.goban.xy(c))
            .map(|(x, y)| GoAction::Cell(x, y))
            .collect_vec()
    }

    fn next(&mut self, action: &GoAction) {
        match action {
            GoAction::Pass => {}
            GoAction::Cell(x, y) => {
                let cell = self.board.goban.cell(*x, *y);
                self.board.place_stone(cell, self.stone);
            }
        }
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
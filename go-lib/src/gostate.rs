use core::fmt;
use std::cmp::Ordering;

use itertools::Itertools;

use board::grid::Grid;
use board::goboard::GoBoard;
use mcts_lib::state::{GameResult, State};
use stones::stone::Stone;

use crate::action::GoAction;
use crate::constants::GOBAN_SIZE;
use board::stats_board::BoardStats;
use board::stats_color::ColorStats;

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
            let black_territory = ColorStats::count_territory(Stone::Black, &self.board);
            let white_territory = ColorStats::count_territory(Stone::White, &self.board);
            let blacks = &self.board.stats.black.captured + black_territory;
            let whites = &self.board.stats.white.captured + white_territory;

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
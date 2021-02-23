use std::fmt;
use std::fmt::{Display, Formatter};

use ordered_float::OrderedFloat;

use state::GameResult;

const _WIN_SCORE: f32 = 1.0;
const _DRAW_SCORE: f32 = 0.5;

pub struct MctsStats<A> {
    pub action: Option<A>,
    pub explored: usize,
    pub wins: usize,
    pub draws: usize,
    pub defeats: usize,
}

impl<A> MctsStats<A> {
    pub(crate) fn new(action: Option<A>) -> Self {
        MctsStats {
            action,
            explored: 0,
            wins: 0,
            draws: 0,
            defeats: 0,
        }
    }

    pub fn selection_score(&self, parent_explored: usize) -> OrderedFloat<f32> {
        let xxx = (parent_explored as f32).ln();

        let x = match self.explored {
            0 => xxx.sqrt(),
            n => {
                let w = self.wins as f32;
                let exploitation = w / n as f32;
                let exploration = (2. * xxx / n as f32).sqrt();
                exploitation + exploration
            }
        };
        OrderedFloat(x)
    }

    pub fn update_score(&mut self, res: GameResult) {
        self.explored += 1;
        match res {
            GameResult::Victory => self.wins += 1,
            GameResult::Defeat => self.defeats += 1,
            GameResult::Draw => self.draws += 1
        }
    }
}

impl<A> fmt::Display for MctsStats<A> where
    A: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut action = String::new();
        match &self.action {
            None => action.push_str("pass"),
            Some(a) => action.push_str(&a.to_string())
        }
        write!(f, "[{} | {} tries, {} wins, {} draws, {} defeats]",
               action,
               self.explored,
               self.wins,
               self.draws,
               self.explored - self.wins - self.draws,
        )
    }
}
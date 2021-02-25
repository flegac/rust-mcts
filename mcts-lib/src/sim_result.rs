use std::{fmt, mem};
use std::fmt::Formatter;

use state::GameResult;

pub struct SimResult {
    pub tries: usize,
    pub wins: usize,
    pub draws: usize,
    pub loses: usize,
}

impl SimResult {
    pub fn new() -> SimResult {
        SimResult {
            tries: 0,
            wins: 0,
            draws: 0,
            loses: 0,
        }
    }

    pub fn update(&mut self, result: GameResult) {
        match result {
            GameResult::Win => self.wins += 1,
            GameResult::Lose => self.loses += 1,
            GameResult::Draw => self.draws += 1,
        }
        self.tries += 1;
    }

    pub fn merge(&mut self, other: &SimResult) {
        self.tries += other.tries;
        self.wins += other.wins;
        self.loses += other.loses;
        self.draws += other.draws;
    }

    pub fn swap(&mut self) {
        mem::swap(&mut self.wins, &mut self.loses);
    }
}

impl fmt::Display for SimResult where {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} tries: {} win, {} draw, {} lose",
               self.tries, self.wins, self.draws, self.loses, )
    }
}
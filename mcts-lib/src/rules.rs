use std::fmt::Debug;
use std::hash::Hash;

use policy::policy::Policy;
use sim_result::SimResult;

#[derive(Debug, Copy, Clone)]
pub enum GameResult { Win, Lose, Draw, Undefined }

impl GameResult {
    pub fn switch(&self) -> Self {
        match self {
            GameResult::Win => GameResult::Lose,
            GameResult::Lose => GameResult::Win,
            _ => *self
        }
    }
}

pub trait Action: Copy + Eq + Hash + Debug {}

impl<T: Copy + Eq + Hash + Debug> Action for T {}

pub trait Rules<A: Action> where Self: Clone {
    fn fork(&self) -> Self;

    fn reset(&mut self);
    fn result(&self) -> Option<GameResult>;
    fn actions(&self) -> Vec<A>;
    fn apply_action(&mut self, action: A);
    fn simulation<P: Policy<A, Self>>(&mut self, policy: &P) -> SimResult {
        while !self.result().is_some() {
            let action = policy.select(self);
            self.apply_action(action);
        }
        SimResult::from_game(self.result().unwrap())
    }
}



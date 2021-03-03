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

pub trait Action: Copy + Eq + Hash {}

impl<T: Copy + Eq + Hash> Action for T {}

pub trait State<A: Action> where Self: Clone {
    fn reset(&mut self);
    fn result(&self) -> Option<GameResult>;
    fn actions(&self) -> Vec<A>;
    fn apply(&mut self, action: A);
    fn simulation<P: Policy<A, Self>>(&mut self, policy: &P) -> SimResult {
        while !self.result().is_some() {
            let action = policy.select(self);
            self.apply(action);
        }
        SimResult::from_game(self.result().unwrap())
    }
}



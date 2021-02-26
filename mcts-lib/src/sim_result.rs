use std::{fmt, mem};
use std::fmt::Formatter;
use std::hash::Hash;


use graph_lib::safe_tree::Tree;
use state::GameResult;

pub(crate) type MctsNode<A> = Tree<A, SimResult>;

pub struct SimResult {
    pub tries: usize,
    pub wins: usize,
    pub draws: usize,
    pub loses: usize,
}

impl SimResult {
    pub fn node<A>() -> MctsNode<A>
        where
            A: Copy, A: Eq, A: Hash {
        Tree::new(SimResult::new())
    }


    pub fn new() -> SimResult {
        SimResult {
            tries: 0,
            wins: 0,
            draws: 0,
            loses: 0,
        }
    }

    pub fn from_game(result: GameResult) -> SimResult {
        let mut res = SimResult {
            tries: 0,
            wins: 0,
            draws: 0,
            loses: 0,
        };
        res.update(result);
        res
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


    pub fn is_leaf(&self) -> bool {
        self.tries == 0
    }

    pub fn exploration(&self, child: &Self) -> f32 {
        let xxx = (self.tries as f32).ln();

        let x = match child.tries {
            0 => xxx,
            n => {
                (2. * xxx / n as f32)
            }
        };
        x.sqrt()
    }
}

impl fmt::Display for SimResult where {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} tries: {} win, {} draw, {} lose",
               self.tries, self.wins, self.draws, self.loses, )
    }
}
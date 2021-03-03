use rust_tools::bench::Bench;

use crate::mcts_tree::mcts_tree::M2;
use crate::mcts_tree::mcts_indextree::M1;

// pub const TREE_SIZE: usize = 10_000;
pub const TREE_SIZE: usize = 20;
pub const BRANCH_FACTOR: usize = 2;

pub trait MCTS<T> {
    fn select_from(&mut self, node: &T) -> T;
    fn expand(&mut self, node: &T, max_children: usize);
}

#[derive(Clone, Debug)]
pub struct MStats {
    pub explored: usize,
    pub wins: usize,
    pub depth: usize,
    pub leafs: usize,
    pub tree_size: usize,
}

impl MStats {
    pub(crate) fn new() -> MStats {
        MStats {
            explored: 0,
            wins: 0,
            depth: 0,
            leafs: 0,
            tree_size: 1,
        }
    }

    pub(crate) fn is_leaf(&self) -> bool {
        self.explored == 0
    }
}

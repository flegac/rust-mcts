pub mod mcts_tree;
mod test_indextree;

// pub const TREE_SIZE: usize = 1_000_000;
pub const TREE_SIZE: usize = 20;
pub const BRANCH_FACTOR: usize = 2;

#[derive(Clone, Debug)]
pub struct MStats {
    explored: usize,
    wins: usize,
    depth: usize,
    childs: usize,
}

impl MStats {
    fn new() -> MStats {
        MStats {
            explored: 0,
            wins: 0,
            depth: 0,
            childs: 0,
        }
    }

    fn is_leaf(&self) -> bool {
        self.explored == 0
    }
}

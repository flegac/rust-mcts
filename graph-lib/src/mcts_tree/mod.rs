pub mod mcts_tree;
mod test_indextree;

pub const TREE_SIZE: usize = 500_000;
// pub const TREE_SIZE: usize = 20;
pub const BRANCH_FACTOR: usize = 2;


pub trait MCTS {
    type Item;

    fn new() -> Self;

    fn size(&self) -> usize;

    fn new_node(&mut self, size: usize) -> Self::Item;

    fn select(&mut self, tree: &Self::Item) -> Self::Item;

    fn expand(&mut self, node: &Self::Item, max_children: usize);

    fn display(&self, node: &Self::Item);

    fn node_size(&self, node: &Self::Item) -> usize;
}

#[derive(Clone, Debug)]
pub struct MStats {
    explored: usize,
    wins: usize,
    depth: usize,
    childs: usize,
    tree_size: usize,
}

impl MStats {
    fn new() -> MStats {
        MStats {
            explored: 0,
            wins: 0,
            depth: 0,
            childs: 0,
            tree_size: 1,
        }
    }

    fn is_leaf(&self) -> bool {
        self.explored == 0
    }
}

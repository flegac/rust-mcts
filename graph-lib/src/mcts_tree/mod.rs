use rust_tools::bench::Bench;

use crate::mcts_tree::mcts_tree::M2;
use crate::mcts_tree::my_indextree::M3;
use crate::mcts_tree::test_indextree::M1;

pub mod mcts_tree;
pub mod test_indextree;
pub mod my_indextree;

// pub const TREE_SIZE: usize = 10_000;
pub const TREE_SIZE: usize = 20;
pub const BRANCH_FACTOR: usize = 2;

#[derive(Copy, Clone, Debug)]
pub struct NodeId {
    index: usize
}

impl NodeId {
    fn new(index: usize) -> NodeId {
        NodeId { index }
    }
}


pub trait MCTS {
    type Item;

    fn node_count(&self) -> usize;

    fn new_node(&mut self, size: usize) -> Self::Item;

    fn select_from(&mut self, node: &Self::Item) -> Self::Item;

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

#[test]
fn test_it() {
    test_mcts(M1::new());
    test_mcts(M2::new());
    test_mcts(M3::new());
}

pub fn test_mcts<T: MCTS>(mut mcts: T) {
    let root = mcts.new_node(BRANCH_FACTOR);

    let mut bench = Bench::new();
    while bench.until_condition(mcts.node_count() >= TREE_SIZE) {
        let selected = mcts.select_from(&root);
        mcts.expand(&selected, BRANCH_FACTOR);
    }

    if mcts.node_count() < 30 {
        mcts.display(&root);
    }
    println!("{} nodes", mcts.node_count());
    println!("{}", bench);
}

use rust_tools::bench::Bench;

use crate::mcts_tree::mcts::{BRANCH_FACTOR, MCTS, TREE_SIZE};
use crate::mcts_tree::mcts_tree::M2;
use crate::mcts_tree::mcts_myindex::M3;
use crate::mcts_tree::mcts_indextree::M1;
use crate::mcts_tree::tree::Tree;

pub mod mcts_tree;
pub mod mcts_indextree;
pub mod mcts_myindex;
mod mcts;
mod tree;


#[test]
fn run_all_mcts() {
    println!("Test M1");
    let m1 = M1::new();
    test_mcts(m1);

    println!("Test M2");
    let m2 = M2::new();
    test_mcts(m2);

    println!("Test M3");
    // let m3 = M3::new();
    // test_mcts<M3>(m3);
}

fn test_mcts<T, S: MCTS<T> + Tree<T>>(mut mcts: S) {
    let root = mcts.new_node(BRANCH_FACTOR);
    let mut bench = Bench::new("Mcts");
    while bench.until_condition(mcts.node_count() >= TREE_SIZE) {
        let selected = mcts.select_from(&root);
        mcts.expand(&selected, BRANCH_FACTOR);
    }

    if mcts.node_count() < 30 {
        // mcts.display(&root);
    }
    println!("{} nodes", mcts.node_count());
    println!("{}", bench);
}

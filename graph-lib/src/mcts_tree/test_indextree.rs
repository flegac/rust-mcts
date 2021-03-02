use std::borrow::{Borrow, BorrowMut};
use std::iter::FromIterator;
use std::time::Duration;

use indextree::{Arena, Node, NodeId};
use rand::prelude::IteratorRandom;

use rust_tools::bench::Bench;

use crate::mcts_tree::{BRANCH_FACTOR, MStats, TREE_SIZE};

struct M {
    arena: Arena<MStats>
}

impl M {
    fn new() -> M {
        M { arena: Arena::new() }
    }

    fn size(&self) -> usize {
        self.arena.count()
    }

    fn new_node(&mut self) -> NodeId {
        self.arena.new_node(MStats::new())
    }

    fn select(&mut self, node: NodeId) -> NodeId {
        match self.arena.get_mut(node) {
            None => panic!(),
            Some(mut n) => {
                if n.get().is_leaf() {
                    n.get_mut().explored += 1;
                    node
                } else {
                    let mut rng = rand::thread_rng();
                    n.get_mut().explored += 1;
                    match node.children(&self.arena).choose(&mut rng) {
                        None => panic!(),
                        Some(child) => {
                            self.select(child)
                        }
                    }
                }
            }
        }
    }

    fn expand(&mut self, node: NodeId, max_children: usize) {
        for _ in 0..max_children {
            let xx = self.expand_one(node, max_children);
        }
    }

    fn expand_one(&mut self, node: NodeId, max_children: usize) -> NodeId {
        let child = self.arena.new_node(MStats::new());
        node.append(child, &mut self.arena);
        let depth = match self.arena.get_mut(node) {
            None => 0,
            Some(nn) => {
                nn.get_mut().childs += 1;
                nn.get().depth
            }
        };

        match self.arena.get_mut(child) {
            None => {}
            Some(nn) => {
                nn.get_mut().depth = depth + 1;
            }
        }
        child
    }

    fn display(&self, node: NodeId) {
        match self.arena.get(node) {
            None => {}
            Some(nn) => {
                let the_node = nn.get();
                let tab = String::from_iter(vec![' '; the_node.depth].iter());
                print!("{}{}:{}", tab, node, the_node.childs);
                for child in node.children(&self.arena) {
                    match self.arena.get(child) {
                        None => print!("X"),
                        Some(c) => {
                            print!("\n{}", tab);
                            self.display(child);
                        }
                    };
                }
            }
        }
    }
}


#[test]
fn test_it() {
    let mut mcts = M::new();
    let root = mcts.new_node();

    let mut bench = Bench::new();
    while bench.until_condition(mcts.size() >= TREE_SIZE) {
        let selected = mcts.select(root);
        mcts.expand(selected, BRANCH_FACTOR);
    }

    println!("{:?}", mcts.display(root));
    println!("{} nodes", mcts.size());
    println!("{}", bench);
}


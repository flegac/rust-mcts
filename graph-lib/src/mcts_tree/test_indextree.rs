use std::borrow::{Borrow, BorrowMut};
use std::iter::FromIterator;
use std::time::Duration;

use indextree::{Arena, Node, NodeId};
use rand::prelude::IteratorRandom;

use rust_tools::bench::Bench;

use crate::mcts_tree::{BRANCH_FACTOR, MCTS, MStats, TREE_SIZE};

struct M {
    arena: Arena<MStats>
}

impl MCTS for M {
    type Item = NodeId;

    fn new() -> M {
        M { arena: Arena::new() }
    }

    fn size(&self) -> usize {
        self.arena.count()
    }

    fn new_node(&mut self, size: usize) -> Self::Item {
        self.arena.new_node(MStats::new())
    }

    fn select(&mut self, node: &Self::Item) -> Self::Item {
        match self.arena.get_mut(node.clone()) {
            None => panic!(),
            Some(mut n) => {
                if n.get().is_leaf() {
                    n.get_mut().explored += 1;
                    node.clone()
                } else {
                    let mut rng = rand::thread_rng();
                    n.get_mut().explored += 1;
                    match node.children(&self.arena).choose(&mut rng) {
                        None => panic!(),
                        Some(child) => {
                            self.select(&child)
                        }
                    }
                }
            }
        }
    }

    fn expand(&mut self, node: &Self::Item, max_children: usize) {
        for _ in 0..max_children {
            let xx = self.expand_one(node.clone(), max_children);
        }
    }

    fn display(&self, node: &Self::Item) {
        match self.arena.get(node.clone()) {
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
                            self.display(&child);
                        }
                    };
                }
            }
        }
    }

    fn node_size(&self, node: &Self::Item) -> usize {
        match self.arena.get(node.clone()) {
            None => 0,
            Some(n) => n.get().childs,
        }
    }
}

impl M {
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
}


#[test]
fn test_it() {
    let mut mcts = M::new();
    let root = mcts.new_node(BRANCH_FACTOR);

    let mut bench = Bench::new();
    while bench.until_condition(mcts.size() >= TREE_SIZE) {
        let selected = mcts.select(&root);
        mcts.expand(&selected, BRANCH_FACTOR);
    }

    println!("{:?}", mcts.display(&root));
    println!("{} nodes", mcts.size());
    println!("{}", bench);
}


use std::borrow::{Borrow, BorrowMut};
use std::iter::FromIterator;
use std::time::Duration;

use indextree::{Arena, Node, NodeId};
use rand::prelude::IteratorRandom;

use rust_tools::bench::Bench;

use crate::mcts_tree::mcts::{BRANCH_FACTOR, MCTS, TREE_SIZE};
use crate::mcts_tree::mcts::MStats;
use crate::mcts_tree::tree::Tree;

pub struct M1 {
    arena: Arena<MStats>
}

impl Tree<NodeId> for M1 {
    fn display(&self, node: &NodeId) {
        log::trace!("display...");

        match self.arena.get(node.clone()) {
            None => {}
            Some(nn) => {
                let the_node = nn.get();
                let tab = String::from_iter(vec![' '; the_node.depth].iter());
                print!("{}{}:{}", tab, node, the_node.leafs);
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
    fn node_count(&self) -> usize {
        self.arena.count()
    }

    fn new_node(&mut self, size: usize) -> NodeId {
        self.arena.new_node(MStats::new())
    }


    fn node_size(&self, node: &NodeId) -> usize {
        match self.arena.get(node.clone()) {
            None => 0,
            Some(n) => n.get().leafs,
        }
    }

    fn set_child(&mut self, i: usize, parent: &NodeId, child: &NodeId) {
        match parent.children(&mut self.arena).find(|c| c == child) {
            None => {
                parent.append(child.clone(), &mut self.arena);
            }
            Some(node) => {
            }
        }
    }
}

impl MCTS<NodeId> for M1 {
    fn select_from(&mut self, node: &NodeId) -> NodeId {
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
                            self.select_from(&child)
                        }
                    }
                }
            }
        }
    }

    fn expand(&mut self, node: &NodeId, max_children: usize) {
        for _ in 0..max_children {
            let xx = self.expand_one(node.clone(), max_children);
        }
    }
}


impl M1 {
    pub fn new() -> M1 {
        M1 { arena: Arena::new() }
    }

    fn expand_one(&mut self, node: NodeId, max_children: usize) -> NodeId {
        let child = self.arena.new_node(MStats::new());
        node.append(child, &mut self.arena);
        let depth = match self.arena.get_mut(node) {
            None => 0,
            Some(nn) => {
                nn.get_mut().leafs += 1;
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


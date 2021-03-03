use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::{FromIterator, Once};
use std::ops::{Deref, DerefMut, Range};
use std::rc::Rc;
use std::time::Duration;

use rand::Rng;

use rust_tools::bench::Bench;

use crate::mcts_tree::{BRANCH_FACTOR, MCTS, MStats, NodeId, test_mcts, TREE_SIZE};

pub struct M3<T> {
    id_gen: usize,
    arena: Vec<T>,
}

impl MCTS for M3<Node> {
    type Item = NodeId;


    fn node_count(&self) -> usize {
        self.id_gen
    }

    fn new_node(&mut self, size: usize) -> Self::Item {
        let id = self.arena.len();

        self.arena.push(Node::new(id, size));
        NodeId::new(id)
    }

    fn select_from(&mut self, tree: &Self::Item) -> Self::Item {
        let mut cur = tree.clone();
        loop {
            let n = &mut self.arena[cur.index];
            if n.data.is_leaf() {
                n.data.explored += 1;
                return cur.clone();
            }

            let mut rng = rand::thread_rng();
            n.data.explored += 1;
            let childs = n.children.len();
            let index = rng.gen_range(0..childs);
            cur = NodeId::new(n.children[index].index);
        }
    }

    fn expand(&mut self, node: &Self::Item, max_children: usize) {
        let n = &self.arena[node.index];
        let nn = n.children.len();
        for i in 0..nn {
            let child = self.new_node(max_children);
            self.set_child(node.clone(), i, child);
        }
    }

    fn display(&self, node: &Self::Item) {
        let nn = &self.arena[node.index];
        let tab = String::from_iter(vec![' '; nn.data.depth].iter());
        print!("{}{}:{}", tab, node.index, nn.data.childs);
        for child in nn.children.iter() {
            let c = &self.arena[child.index];
            print!("\n{}", tab);
            self.display(&child);
        }
    }
    fn node_size(&self, node: &Self::Item) -> usize {
        let n = &self.arena[node.index];
        return n.data.tree_size;
    }
}

impl M3<Node> {
    pub fn new() -> M3<Node> {
        M3 {
            id_gen: 0,
            arena: Vec::new(),
        }
    }

    fn set_child(&mut self, parent: NodeId, i: usize, child: NodeId) {

        // update child depth
        {
            let depth = self.arena[parent.index].data.depth;
            let c = &mut self.arena[child.index];
            c.data.depth = depth + 1;
        }

        let tree_size = self.arena[parent.index]
            .children
            .iter()
            .map(|c| self.arena[c.index].data.tree_size.clone())
            .sum();
        let p = &mut self.arena[parent.index];
        if p.children[i].index == 0 { //if  LEAF
            p.data.childs -= 1;
        }
        p.children[i] = child;
        p.data.tree_size = tree_size;
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    index: usize,
    data: MStats,
    children: Vec<NodeId>,
}


impl Node {
    fn new(id: usize, size: usize) -> Node {
        let mut stats = MStats::new();
        stats.childs = size;
        let mut res = Node {
            index: id,
            data: stats,
            children: Vec::with_capacity(size),
        };
        res.children.resize(size, NodeId::new(0));
        res
    }
}


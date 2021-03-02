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

use crate::mcts_tree::{BRANCH_FACTOR, MStats, TREE_SIZE};

struct M {
    id_gen: usize
}

impl M {
    fn new() -> M {
        M { id_gen: 0 }
    }

    fn size(&self) -> usize {
        self.id_gen
    }

    fn new_node(&mut self, size: usize) -> Tree {
        let id = self.id_gen;
        self.id_gen += 1;
        let mut stats = MStats::new();
        stats.childs = size;
        Some(Rc::new(RefCell::new(Node {
            id,
            value: stats,
            children: vec![None; size],
        })))
    }

    fn select(&self, tree: &Tree) -> Tree {
        match tree {
            None => panic!(),
            Some(n) => {
                if n.as_ref().borrow().value.is_leaf() {
                    n.as_ref().borrow_mut().value.explored += 1;
                    tree.clone()
                } else {
                    let mut rng = rand::thread_rng();
                    n.as_ref().borrow_mut().value.explored += 1;
                    let childs = n.as_ref().borrow().children.len();
                    let index = rng.gen_range(0..childs);
                    self.select(&n.as_ref().borrow().children[index])
                }
            }
        }
    }

    fn expand(&mut self, node: &Tree, max_children: usize) {
        match &node {
            None => panic!(),
            Some(node) => {
                let nn = node.as_ref().borrow().children.len();
                for i in 0..nn {
                    let child = self.new_node(max_children);
                    node.as_ref().borrow_mut().set_child(i, child);
                }
            }
        }
    }

    fn display(&self, node: &Tree) {
        let data = &node.clone().map(|x| x.as_ref().borrow().to_string());
        match data {
            None => {}
            Some(data) => {
                println!("{}", data);
            }
        }
    }

    fn node_size(node: &Tree) -> usize {
        match node {
            None => 0,
            Some(n) => n.as_ref().borrow().size(),
        }
    }
}


type Tree = Option<Rc<RefCell<Node>>>;

#[derive(Clone, Debug)]
struct Node {
    id: usize,
    value: MStats,
    children: Vec<Tree>,
}

impl Node {
    fn size(&self) -> usize {
        let cpt: usize = self
            .children
            .iter()
            .map(|c| match c {
                None => 1,
                Some(n) => n.as_ref().borrow().size(),
            })
            .sum();
        return cpt + 1;
    }

    fn display(&self, node: &Tree) {
        let data = &node.clone().map(|x| x.as_ref().borrow().to_string());
        match data {
            None => {}
            Some(data) => {
                println!("{}", data);
            }
        }
    }

    fn set_child(&mut self, i: usize, child: Tree) {
        // update child depth
        match &child {
            None => {}
            Some(n) => {
                n.as_ref().borrow_mut().value.depth = self.value.depth + 1;
                // n.as_ref().borrow_mut().depth = self.depth + 1;
            }
        }

        match &self.children[i] {
            None => {
                self.value.childs -= 1;
            }
            _ => {}
        }
        self.children[i] = child;
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let tab = String::from_iter(vec![' '; self.value.depth].iter());
        write!(f, "{}{}:{}", tab, self.id, self.value.childs);
        if !(self.value.is_leaf()) {
            for child in self.children.iter() {
                match child {
                    None => write!(f, "X"),
                    Some(c) => write!(f, "\n{}{}", tab, c.as_ref().borrow()),
                };
            }
        }

        Ok(())
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

    mcts.display(&root);
    println!("{} nodes", mcts.size());
    println!("{}", bench);
}

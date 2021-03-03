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

use crate::mcts_tree::{BRANCH_FACTOR, MCTS, MStats, TREE_SIZE};

struct M<T> {
    id_gen: usize,
    arena: Vec<Rc<RefCell<T>>>,
}

impl MCTS for M<Node> {
    type Item = Tree;


    fn node_count(&self) -> usize {
        self.id_gen
    }

    fn new_node(&mut self, size: usize) -> Self::Item {
        let id = self.id_gen;
        self.id_gen += 1;
        let node = Rc::new(RefCell::new(Node::new(id, size)));
        //TODO: use the arena strategy
        Some(node)
    }

    fn select_from(&mut self, tree: &Self::Item) -> Self::Item {
        match tree {
            None => panic!(),
            Some(n) => {
                if n.as_ref().borrow().data.is_leaf() {
                    n.as_ref().borrow_mut().data.explored += 1;
                    tree.clone()
                } else {
                    let mut rng = rand::thread_rng();
                    n.as_ref().borrow_mut().data.explored += 1;
                    let childs = n.as_ref().borrow().children.len();
                    let index = rng.gen_range(0..childs);
                    self.select_from(&n.as_ref().borrow().children[index])
                }
            }
        }
    }

    fn expand(&mut self, node: &Self::Item, max_children: usize) {
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

    fn display(&self, node: &Self::Item) {
        let data = &node.clone().map(|x| x.as_ref().borrow().to_string());
        match data {
            None => {}
            Some(data) => {
                println!("{}", data);
            }
        }
    }

    fn node_size(&self, node: &Self::Item) -> usize {
        match node {
            None => 0,
            Some(n) => n.as_ref().borrow().size(),
        }
    }
}

impl M<Node> {
    fn new() -> M<Node> {
        M {
            id_gen: 0,
            arena: Vec::new(),
        }
    }
}

//TODO: change to Rc<RefCell<Option<Node>>> ?? (this would make the data easily clonable)
type Tree = Option<Rc<RefCell<Node>>>;

#[derive(Clone, Debug)]
struct Node {
    id: usize,
    data: MStats,
    children: Vec<Tree>,
}


impl Node {
    fn new(id: usize, size: usize) -> Node {
        let mut stats = MStats::new();
        stats.childs = size;
        Node {
            id,
            data: stats,
            children: vec![None; size],
        }
    }

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

    fn set_child(&mut self, i: usize, child: Tree) {
        // update child depth
        match &child {
            None => {}
            Some(n) => {
                n.as_ref().borrow_mut().data.depth = self.data.depth + 1;
                // n.as_ref().borrow_mut().depth = self.depth + 1;
            }
        }

        match &self.children[i] {
            None => {
                self.data.childs -= 1;
            }
            _ => {}
        }
        self.children[i] = child;
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let tab = String::from_iter(vec![' '; self.data.depth].iter());
        write!(f, "{}{}:{}", tab, self.id, self.data.childs);
        if !(self.data.is_leaf()) {
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

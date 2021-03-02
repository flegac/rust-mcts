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

#[derive(Clone, Debug)]
struct Node {
    id: usize,
    depth: usize,
    leafs: usize,
    children: Vec<Tree>,
}

impl Node {
    fn is_leaf(&self) -> bool {
        self.leafs == self.children.len()
    }

    fn size(&self) -> usize {
        let cpt: usize = self
            .children
            .iter()
            .map(|c| match c {
                Tree::Leaf => 1,
                Tree::Node(n) => n.as_ref().borrow().size(),
            })
            .sum();
        return cpt + 1;
    }

    fn set_child(&mut self, i: usize, child: Tree) {
        // update child depth
        match &child {
            Tree::Leaf => {}
            Tree::Node(n) => {
                n.as_ref().borrow_mut().depth = self.depth + 1;
            }
        }

        match &self.children[i] {
            Tree::Leaf => {
                self.leafs -= 1;
            }
            _ => {}
        }
        self.children[i] = child;
    }
}

#[derive(Clone, Debug)]
enum Tree {
    Leaf,
    Node(Rc<RefCell<Node>>),
}

impl Tree {
    fn node(id: usize, size: usize) -> Tree {
        Tree::Node(Rc::new(RefCell::new(Node {
            id: id,
            depth: 0,
            leafs: size,
            children: vec![Tree::Leaf; size],
        })))
    }

    fn select(&self) -> Tree {
        match self {
            Tree::Leaf => panic!(),
            Tree::Node(node) => {
                if node.as_ref().borrow().is_leaf() {
                    self.clone()
                } else {
                    //TODO : smarter selection
                    let childs = node.as_ref().borrow().children.len();
                    let mut rng = rand::thread_rng();
                    let index = rng.gen_range(0..childs);
                    node.as_ref().borrow().children[index].select()
                }
            }
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let tab = String::from_iter(vec![' '; self.depth].iter());
        write!(f, "{}{}:{}", tab, self.id, self.leafs);
        if !(self.is_leaf()) {
            for child in self.children.iter() {
                match child {
                    Tree::Leaf => write!(f, "X"),
                    Tree::Node(_) => write!(f, "\n{}{}", tab, child),
                };
            }
        }

        Ok(())
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Tree::Leaf => write!(f, "X"),
            Tree::Node(node) => write!(f, "{}", node.as_ref().borrow()),
        }
    }
}

#[test]
fn test_it() {
    let mut id_gen = 0;
    let root = Tree::node(id_gen, 19 * 19);
    id_gen += 1;

    let mut bench = Bench::new(Duration::from_secs(1));
    while bench.looping_inc(None) {
        let selected = root.select();
        match selected {
            Tree::Leaf => panic!(),
            Tree::Node(node) => {
                let nn = node.as_ref().borrow().children.len();
                for i in 0..nn {
                    let tree = Tree::node(id_gen, 19 * 19);
                    id_gen += 1;
                    node.as_ref().borrow_mut().set_child(i, tree);
                }
            }
        }
    }
    // println!("{}", root);
    match root {
        Tree::Leaf => {}
        Tree::Node(node) => {
            println!("{} nodes", node.as_ref().borrow().size());
        }
    }

    println!("{}\n{}", bench, bench.log_speed(1 as f32));
}

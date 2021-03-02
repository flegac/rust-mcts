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
struct Mcts {
    explored: usize,
    wins: usize,
}

type Tree = Option<Rc<RefCell<Node>>>;

#[derive(Clone, Debug)]
struct Node {
    id: usize,
    value: Mcts,
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
                n.as_ref().borrow_mut().depth = self.depth + 1;
            }
        }

        match &self.children[i] {
            None => {
                self.leafs -= 1;
            }
            _ => {}
        }
        self.children[i] = child;
    }
}


fn select(tree: &Tree) -> Tree {
    match tree {
        None => panic!(),
        Some(node) => {
            if node.as_ref().borrow().is_leaf() {
                tree.clone()
            } else {
                //TODO : smarter selection
                let childs = node.as_ref().borrow().children.len();
                let mut rng = rand::thread_rng();
                let index = rng.gen_range(0..childs);
                select(&node.as_ref().borrow().children[index])
            }
        }
    }
}

struct M {}

impl M {
    fn node(id: usize, size: usize) -> Tree {
        Some(Rc::new(RefCell::new(Node {
            id: id,
            value: Mcts { explored: 0, wins: 0 },
            depth: 0,
            leafs: size,
            children: vec![None; size],
        })))
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let tab = String::from_iter(vec![' '; self.depth].iter());
        write!(f, "{}{}:{}", tab, self.id, self.leafs);
        if !(self.is_leaf()) {
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
    let mut id_gen = 0;
    let root = M::node(id_gen, 19 * 19);
    id_gen += 1;

    let mut bench = Bench::new(Duration::from_secs(1));
    while bench.looping_inc(None) {
        let selected = select(&root);
        match selected {
            None => panic!(),
            Some(node) => {
                let nn = node.as_ref().borrow().children.len();
                for i in 0..nn {
                    let tree = M::node(id_gen, 19 * 19);
                    id_gen += 1;
                    node.as_ref().borrow_mut().set_child(i, tree);
                }
            }
        }
    }
    // println!("{}", root);
    match root {
        None => {}
        Some(node) => {
            println!("{} nodes", node.as_ref().borrow().size());
        }
    }

    println!("{}\n{}", bench, bench.log_speed(1 as f32));
}

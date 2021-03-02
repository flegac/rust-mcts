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

trait Mcts {
    fn is_leaf(&self) -> bool;
}

#[derive(Clone, Debug)]
struct MStats {
    explored: usize,
    wins: usize,
}

type Tree = Option<Rc<RefCell<Node>>>;

#[derive(Clone, Debug)]
struct Node {
    id: usize,
    value: MStats,
    depth: usize,
    leafs: usize,
    children: Vec<Tree>,
}

impl Mcts for Node {
    fn is_leaf(&self) -> bool {
        self.value.explored == 0
    }
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
                node.as_ref().borrow_mut().value.explored+=1;

                tree.clone()
            } else {
                node.as_ref().borrow_mut().value.explored+=1;

                //TODO : smarter selection
                let childs = node.as_ref().borrow().children.len();
                let mut rng = rand::thread_rng();
                let index = rng.gen_range(0..childs);
                select(&node.as_ref().borrow().children[index])
            }
        }
    }
}

struct M {
    id_gen: usize
}

impl M {
    fn new() -> M {
        M { id_gen: 0 }
    }
    fn node(&mut self, size: usize) -> Tree {
        let id = self.id_gen;
        self.id_gen += 1;
        Some(Rc::new(RefCell::new(Node {
            id,
            value: MStats { explored: 0, wins: 0 },
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
    let branch_factor = 3;
    let mut mcts = M::new();
    let root = mcts.node(branch_factor);

    let mut bench = Bench::new(Duration::from_millis(2));
    while bench.looping_inc(None) {
        let selected = select(&root);
        match &selected {
            None => panic!(),
            Some(node) => {
                let nn = node.as_ref().borrow().children.len();
                for i in 0..nn {
                    let tree = mcts.node(branch_factor);
                    node.as_ref().borrow_mut().set_child(i, tree);
                }
            }
        }
    }
    match root {
        None => panic!(),
        Some(node) => {
            println!("{}", node.as_ref().borrow());
            println!("{} nodes", node.as_ref().borrow().size());
        }
    }

    println!("{}\n{}", bench, bench.log_speed(1 as f32));
}

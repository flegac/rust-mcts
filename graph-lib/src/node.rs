use std::cell::RefCell;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::rc::{Rc, Weak};

use crate::safe_tree::SafeTree;

pub type NodeRc<T> = Rc<Node<T>>;
pub type NodeWeak<T> = Weak<Node<T>>;

pub struct Node<T> {
    pub value: RefCell<T>,
    pub parent: RefCell<NodeWeak<T>>,
    pub children: RefCell<Vec<NodeRc<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            value: RefCell::new(value),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        }
    }

    pub fn parent_value(&self) -> Rc<Node<T>> {
        match self.parent.borrow().upgrade() {
            None => panic!(),
            Some(parent) => {
                parent
            }
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children.borrow().is_empty()
    }

    pub fn parents(&self) -> Vec<Rc<Self>> {
        let mut res = vec![];
        let mut t = self.parent
            .borrow()
            .upgrade();
        while t.is_some() {
            let rc = t.unwrap();
            res.push(Rc::clone(&rc));
            t = rc.parent
                .borrow()
                .upgrade();
        }
        res
    }

    pub fn max_by_key<B: Ord, F>(&self, f: F) -> Option<SafeTree<T>>
        where F: Fn(&T) -> B {
        let x = self.children.borrow()
            .iter()
            .max_by_key(|x| f(x.value.borrow().deref()))
            .map(Rc::clone);
        x.map(|rc: Rc<Node<T>>| SafeTree::from_node(rc))
    }

    pub(crate) fn child_at(&self, index: usize) -> Option<Rc<Self>> {
        self.children.borrow().get(index)
            .map(|c| Rc::clone(c))
    }
}

impl<T> fmt::Display for Node<T> where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}: {} childs]",
               self.value.borrow(),
               self.children.borrow().len())
    }
}

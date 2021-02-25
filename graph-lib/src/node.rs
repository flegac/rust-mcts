use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::rc::{Rc, Weak};

use crate::safe_tree::Tree;
use std::hash::Hash;

pub type NodeRc<K, V> = Rc<Node<K, V>>;
pub type NodeWeak<K, V> = Weak<Node<K, V>>;

pub struct Node<K, V> {
    pub value: RefCell<V>,
    pub parent: RefCell<NodeWeak<K, V>>,
    pub children: RefCell<HashMap<K, NodeRc<K, V>>>,
}

impl<K, V> Node<K, V> where K: Eq, K: Hash {
    pub fn new(value: V) -> Node<K, V> {
        Node {
            value: RefCell::new(value),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(HashMap::new()),
        }
    }

    pub fn parent_value(&self) -> Rc<Node<K, V>> {
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

    pub fn max_by_key<B: Ord, F>(&self, f: F) -> Option<Tree<K, V>>
        where F: Fn(&V) -> B {
        let x = self.children.borrow()
            .values()
            .max_by_key(|x| f(x.value.borrow().deref()))
            .map(Rc::clone);
        x.map(|rc: Rc<Node<K, V>>| Tree::from_node(rc))
    }

    pub(crate) fn child_at(&self, index: &K) -> Option<Rc<Self>> {
        self.children.borrow().get(index)
            .map(|c| Rc::clone(c))
    }
}

impl<K, V> fmt::Display for Node<K, V> where V: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}: {} childs]",
               self.value.borrow(),
               self.children.borrow().len())
    }
}

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::rc::{Rc, Weak};

use crate::safe_tree::NodeRc;

type NodeWeak<K, V> = Weak<Node<K, V>>;

pub struct Node<K, V> {
    pub value: RefCell<V>,
    pub parent: RefCell<Option<(K, NodeWeak<K, V>)>>,
    pub children: RefCell<HashMap<K, NodeRc<K, V>>>,
    pub depth: RefCell<usize>
}

impl<K, V> Node<K, V>
where
    K: Copy,
    K: Eq,
    K: Hash,
{
    pub fn new(value: V) -> Node<K, V> {
        Node {
            value: RefCell::new(value),
            parent: RefCell::new(None),
            children: RefCell::new(HashMap::new()),
            depth:RefCell::new(0)
        }
    }

    pub fn parent_value(&self) -> Option<(K, NodeRc<K, V>)> {
        self.parent
            .borrow()
            .as_ref()
            .map(|(key, value)| (key.clone(), value.upgrade().unwrap()))
    }

    pub fn is_leaf(&self) -> bool {
        self.children.borrow().is_empty()
    }

    pub(crate) fn child_at(&self, index: K) -> Option<Rc<Self>> {
        self.children.borrow().get(&index).map(|c| Rc::clone(c))
    }
}

impl<K, V> fmt::Display for Node<K, V>
where
    V: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}: {} childs]",
            self.value.borrow(),
            self.children.borrow().len()
        )
    }
}

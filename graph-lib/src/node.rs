use std::cell::RefCell;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::{Rc, Weak};

pub struct Node<T> {
    pub value: RefCell<T>,
    pub parent: RefCell<Weak<Node<T>>>,
    pub children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn is_leaf(&self) -> bool {
        self.children.borrow().is_empty()
    }
}

impl<T> fmt::Display for Node<T> where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}: {} childs]",
               self.value.borrow(),
               self.children.borrow().len())
    }
}

use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::rc::{Rc, Weak};

use trees::node::Node;

pub struct Tree<T>(Rc<Node<T>>);


impl<T> Tree<T> {
    pub fn clone(&self) -> Tree<T> {
        Tree(Rc::clone(&self.0))
    }

    pub fn new(value: T) -> Tree<T> {
        Tree(Rc::new(Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        }))
    }

    pub fn parent(&self) -> Option<Tree<T>> {
        self.parent.borrow().upgrade().map(|c| Tree(Rc::clone(&c)))
    }

    pub fn value(&self) -> &T {
        &self.0.value
    }

    // pub fn value_mut(&mut self) -> &mut T {
    //     &mut self.borrow_mut().value
    // }

    pub fn set_child(&self, index: usize, value: &Tree<T>) {
        self.0.children.borrow_mut().as_mut_slice()[index] = Rc::clone(&value.0);
    }

    pub fn remove(&self, index: usize) {
        self.0.children.borrow_mut().remove(index);
    }

    pub fn get_child(&self, index: usize) -> Option<Tree<T>> {
        self.0.children.borrow().get(index)
            .map(|c| Tree(Rc::clone(c)))
    }

    pub fn add_child(&self, tree: &Tree<T>) {
        self.0.children.borrow_mut().push(Rc::clone(&tree.0));
        *tree.0.parent.borrow_mut() = Rc::downgrade(&self.0);
    }
}

impl<T> Deref for Tree<T> {
    type Target = Rc<Node<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> fmt::Display for Tree<T>
    where T: Display, T: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{}", self.0))
    }
}

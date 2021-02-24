use core::cell::RefCell;
use core::fmt;
use core::fmt::{Display, Formatter};
use core::option::Option;
use std::ops::Deref;
use std::rc::{Rc, Weak};

use crate::node::Node;
use crate::tree::Tree;

pub struct SafeTree<T>(Rc<Node<T>>);

impl<T> SafeTree<T> {
    pub fn clone(&self) -> SafeTree<T> {
        SafeTree(Rc::clone(&self.0))
    }

    pub fn from_node(node: Rc<Node<T>>) -> SafeTree<T> {
        SafeTree(node)
    }

    pub fn new(value: T) -> SafeTree<T> {
        SafeTree(Rc::new(Node {
            value: RefCell::new(value),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        }))
    }

    pub fn max_by_key<B: Ord, F>(&self, f: F) -> Option<SafeTree<T>>
        where F: Fn(&T) -> B {
        let x = self.children.borrow()
            .iter()
            .max_by_key(|x| f(x.value.borrow().deref()))
            ?.clone();
        Some(SafeTree::from_node(x))
    }
}

impl<T> Deref for SafeTree<T> {
    type Target = Rc<Node<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> fmt::Display for SafeTree<T>
    where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{}", self.0))
    }
}

impl<T> Tree for SafeTree<T> {
    fn parent(&self) -> Option<Self> {
        self.parent.borrow().upgrade().map(|c| SafeTree(Rc::clone(&c)))
    }


    fn set_child(&self, index: usize, value: &Self) {
        self.0.children.borrow_mut().as_mut_slice()[index] = Rc::clone(&value.0);
    }

    fn remove(&self, index: usize) {
        self.0.children.borrow_mut().remove(index);
    }

    fn get_child(&self, index: usize) -> Option<Self> {
        self.0.children.borrow().get(index)
            .map(|c| SafeTree(Rc::clone(c)))
    }

    fn parents(&self) -> Vec<Self> {
        let mut res = vec![];
        let mut t = self.clone();
        while t.parent().is_some() {
            t = t.parent().unwrap().clone();
            res.push(t.clone());
        }
        res
    }


    fn add_child(&self, tree: &Self) {
        self.0.children.borrow_mut().push(Rc::clone(&tree.0));
        *tree.0.parent.borrow_mut() = Rc::downgrade(&self.0);
    }
}

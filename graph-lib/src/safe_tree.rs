use core::fmt;
use core::fmt::{Display, Formatter};
use core::option::Option;
use std::ops::Deref;
use std::rc::Rc;

use crate::node::{Node, NodeRc};
use crate::tree::Tree;

pub struct SafeTree<T>(NodeRc<T>);

impl<T> SafeTree<T> {
    pub fn clone(&self) -> SafeTree<T> {
        SafeTree(Rc::clone(&self.0))
    }

    pub fn from_node(node: NodeRc<T>) -> SafeTree<T> {
        SafeTree(node)
    }

    pub fn new(value: T) -> SafeTree<T> {
        SafeTree(Rc::new(Node::new(value)))
    }
}

impl<T> Deref for SafeTree<T> {
    type Target = NodeRc<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> fmt::Display for SafeTree<T>
    where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> Tree<usize> for SafeTree<T> {
    fn parent(&self) -> Option<Self> {
        self.parent.borrow().upgrade().map(|c| SafeTree(Rc::clone(&c)))
    }

    fn set_child(&self, index: usize, value: &Self) {
        self.0.children.borrow_mut().as_mut_slice()[index] = Rc::clone(&value.0);
    }

    fn remove(&self, index: usize) {
        self.0.children.borrow_mut().remove(index);
    }

    fn add_child(&self, tree: &Self) {
        self.0.children.borrow_mut().push(Rc::clone(&tree.0));
        *tree.0.parent.borrow_mut() = Rc::downgrade(&self.0);
    }

}

#[test]
fn test_it() {
    let root = SafeTree::new(1);

    // root.add_child(&SafeTree::new(10));
    // root.add_child(&SafeTree::new(11));
    // root.add_child(&SafeTree::new(12));
    // println!("{}", &root);
    // root.get_child(1).map(|c| {
    //     c.add_child(&SafeTree::new(110));
    //     c.add_child(&SafeTree::new(111));
    //     c.add_child(&SafeTree::new(112));
    // });
    // println!("{}", &root);
    // root.remove(2);
    // println!("{}", &root);
    // // root.set_child(0, &root.get_child(1).unwrap());
    // println!("{}", &root);
    //
    // let c = root.get_child(0).unwrap().get_child(0).unwrap();
    //
    //
    // println!("parents({}) :", c);
    // for x in c.parents() {
    //     println!("- {}", x);
    // }
}

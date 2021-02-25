use core::fmt;
use core::fmt::{Display, Formatter};
use core::option::Option;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::hash::Hash;
use std::ops::Deref;
use std::rc::Rc;

use crate::node::{Node, NodeRc};

pub struct Tree<K, V> (NodeRc<K, V>) where K: Eq, K: Hash;

impl<K, V> Tree<K, V> where K: Copy, K: Eq, K: Hash {
    pub fn clone(&self) -> Tree<K, V> {
        let x = &self.0;
        let y = Rc::clone(x);

        Tree::from_node(y)
    }

    pub fn from_node(node: NodeRc<K, V>) -> Tree<K, V> {
        Tree(node)
    }

    pub fn new(value: V) -> Tree<K, V> {
        Tree(Rc::new(Node::new(value)))
    }


    pub fn parents(&self) -> Vec<(K, NodeRc<K, V>)> {
        let mut res = vec![];
        let mut node = self.0.clone();
        let mut x = node.parent_value();
        while let Some((key, value)) = x {
            res.push((key, value.clone()));
            node = value.clone();
            x = node.parent_value();
        }
        res
    }

    pub fn parent(&self) -> Option<(K, Rc<Node<K, V>>)> {
        self.parent
            .borrow()
            .clone()
            .map(|(key, value)| {
                (key, value.upgrade().unwrap())
            })
    }

    pub fn set_child(&self, index: K, value: &Self) {
        self.0.children.borrow_mut().insert(index, Rc::clone(&value.0));
        let data = Rc::downgrade(&self.0);
        value.parent.replace(Some((index, data)));
    }

    // pub fn remove(&self, index: K) {
    //     self.0.children.borrow_mut().remove(&index);
    // }
}

impl<K, V> Deref for Tree<K, V> where K: Eq, K: Hash {
    type Target = NodeRc<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> fmt::Display for Tree<K, V>
    where V: Display,
          K: Eq, K: Hash {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[test]
fn test_it() {
    let root = Tree::new(1);

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

use core::fmt;
use core::fmt::{Display, Formatter};
use core::option::Option;
use std::borrow::Borrow;
use std::hash::Hash;
use std::ops::Deref;
use std::rc::Rc;

use crate::algo::trees::Trees;
use crate::node::Node;
use crate::tree::TheTree;

pub struct Tree<K, V>(NodeRc<K, V>)
    where
        K: Eq,
        K: Hash;

pub type NodeRc<K, V> = Rc<Node<K, V>>;

impl<K, V> Tree<K, V>
    where
        K: Copy,
        K: Eq,
        K: Hash,
{
    pub fn clone(&self) -> Tree<K, V> {
        let x = &self.0;
        let y = Rc::clone(x);

        Tree::from_node(y)
    }

    pub fn new(value: V) -> Self {
        Tree(Rc::new(Node::new(value)))
    }

    pub fn parents(&self) -> Vec<(K, Tree<K, V>)> {
        let mut res = vec![];
        let mut node = self.0.clone();
        let mut x = node.parent_value();
        while let Some((key, value)) = x {
            res.push((key, Tree::from_node(value.clone())));
            node = value.clone();
            x = node.parent_value();
        }
        res
    }

    pub(crate) fn from_node(node: NodeRc<K, V>) -> Tree<K, V> {
        Tree(node)
    }
}

impl<K, V> Trees<K, V> for Tree<K, V>
    where
        K: Copy,
        K: Eq,
        K: Hash,
{
    fn search_max_child<B: Ord, F>(&self, f: F) -> Option<(K, Self)>
        where
            F: Fn(&V) -> B,
    {
        self.0
            .children
            .borrow()
            .iter()
            .max_by_key(|(&_key, value)| f(value.value.borrow().deref()))
            .map(|(k, v)| (k.clone(), Tree::from_node(v.clone())))
    }
}

impl<K, V> TheTree<K, V> for Tree<K, V>
    where
        K: Copy,
        K: Eq,
        K: Hash,
{
    fn parent(&self) -> Option<(K, Self)> {
        self.0
            .parent
            .borrow()
            .clone()
            .map(|(key, value)| (key, Tree::from_node(value.upgrade().unwrap())))
    }

    fn set_child(&self, index: K, value: &Self) {
        self.0
            .children
            .borrow_mut()
            .insert(index, Rc::clone(&value.0));
        let data = Rc::downgrade(&self.0);
        value.0.parent.replace(Some((index, data)));
        value.depth.replace(self.depth.take() + 1);
    }

    fn get_child(&self, index: K) -> Option<Self> {
        self.0.child_at(index).map(Tree::from_node)
    }

    fn remove(&self, index: K) {
        self.0.children.borrow_mut().remove(&index);
    }
}

impl<K, V> Deref for Tree<K, V>
    where
        K: Eq,
        K: Hash,
{
    type Target = NodeRc<K, V>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> fmt::Display for Tree<K, V>
    where
        V: Display,
        K: Eq,
        K: Hash,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[test]
fn test_it() {
    let root = Tree::new(1);

    root.set_child(1, &Tree::new(10));
    root.set_child(2, &Tree::new(11));
    root.set_child(3, &Tree::new(12));
    println!("{}", &root);
    root.get_child(1).map(|c| {
        c.set_child(1, &Tree::new(110));
        c.set_child(2, &Tree::new(111));
        c.set_child(3, &Tree::new(112));
    });
    println!("{}", &root);
    root.remove(2);
    println!("{}", &root);
    // root.set_child(0, &root.get_child(1).unwrap());
    println!("{}", &root);

    let c = root.get_child(1).unwrap().get_child(2).unwrap();

    println!("parents({}) :", c);
    for (key, value) in c.parents() {
        println!("- {} {}", key, value);
    }
}

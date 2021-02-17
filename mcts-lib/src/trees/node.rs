use std::cell::RefCell;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::rc::{Rc, Weak};

pub struct Node<T> {
    pub value: T,
    pub parent: RefCell<Weak<Node<T>>>,
    pub children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> fmt::Display for Node<T> where T: Display, T: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        res.push_str(&self.value.to_string());
        if !self.children.borrow().is_empty() {
            res.push_str("(");
        }
        for (i, child) in self.children.borrow().iter().enumerate() {
            if i == 0 {
                res.push_str(format!("{}", child).as_str());
            } else {
                res.push_str(format!(" {}", child).as_str());
            }
        }
        if !self.children.borrow().is_empty() {
            res.push_str(")");
        }
        write!(f, "{}", res)
    }
}
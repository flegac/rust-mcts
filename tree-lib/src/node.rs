use std::cell::RefCell;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct N {
    pub data: RefCell<usize>,
    pub children: RefCell<Vec<Rc<N>>>,
}



pub struct Node<T> {
    pub value: RefCell<T>,
    pub parent: RefCell<Weak<Node<T>>>,
    pub children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> fmt::Display for Node<T> where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        res.push_str(&self.value.borrow().to_string());
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

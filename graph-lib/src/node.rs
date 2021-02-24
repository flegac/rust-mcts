use std::cell::RefCell;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
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

    pub fn max_by_key<B: Ord, F>(&self, f: F) -> Option<Rc<Node<T>>>
        where F: Fn(&T) -> B {
        let x = self.children.borrow()
            .iter()
            .max_by_key(|x| f(x.value.borrow().deref()))
            .map(Rc::clone);
        x
    }

    fn get_child(&self, index: usize) -> Option<Rc<Self>> {
        self.children.borrow().get(index)
            .map(|c| Rc::clone(c))
    }
}

impl<T> fmt::Display for Node<T> where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}: {} childs]",
               self.value.borrow(),
               self.children.borrow().len())
    }
}

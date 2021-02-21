use std::cell::RefCell;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::rc::{Rc, Weak};

pub trait Tree where Self:Sized {
    fn parent(&self) -> Option<Self>;
    fn set_child(&self, index: usize, value: &Self);
    fn remove(&self, index: usize);
    fn get_child(&self, index: usize) -> Option<Self>;
    fn parents(&self) -> Vec<Self>;
    fn add_child(&self, tree: &Self);
}


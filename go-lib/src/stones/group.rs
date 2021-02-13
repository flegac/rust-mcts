use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::fmt::Formatter;
use std::rc::Rc;

use bit_set::BitSet;

use stones::stone::Stone;

use crate::board::GoCell;
use crate::constants::GOBAN_SIZE;

#[derive(Eq, PartialEq)]
pub(crate) struct GoGroup {
    pub(crate) stone: Stone,
    pub(crate) cells: BitSet,
}

impl GoGroup {
    pub(crate) fn new(stone: Stone) -> GoGroup {
        GoGroup {
            stone,
            cells: BitSet::with_capacity(GOBAN_SIZE * GOBAN_SIZE),
        }
    }

    pub(crate) fn map<F: Fn(GoCell) -> ()>(&self, closure: F) {
        for c in self.cells.iter() {
            closure(c)
        }
    }


    pub(crate) fn add_group(&mut self, other: &GoGroup) {
        self.cells.union_with(&other.cells);
    }

    pub(crate) fn remove_group(&mut self, other: &GoGroup) {
        self.cells.difference_with(&other.cells);
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    pub(crate) fn set_stone(&mut self, stone: Stone) {
        self.stone = stone;
    }

    pub(crate) fn add_cells(&mut self, cells: &[GoCell]) {
        for &c in cells {
            self.cells.insert(c);
        }
    }

    pub(crate) fn remove_cells(&mut self, cells: &[GoCell]) {
        for &c in cells {
            self.cells.remove(c);
        }
    }
}

#[derive(Eq, PartialEq)]
pub struct GoGroupRc(Rc<RefCell<GoGroup>>);

impl GoGroupRc {
    pub(crate) fn new(stone: Stone) -> Self {
        GoGroupRc(Rc::new(RefCell::new(GoGroup::new(stone))))
    }

    pub(crate) fn with_cells(self, cells: &[GoCell]) -> Self {
        for &c in cells {
            self.borrow_mut().cells.insert(c);
        }
        self
    }


    pub(crate) fn from_stones(group: GoGroup) -> Self {
        GoGroupRc(Rc::new(RefCell::new(group)))
    }

    pub(crate) fn clone(&self) -> Self {
        GoGroupRc(Rc::clone(&self.0))
    }

    pub(crate) fn borrow(&self) -> Ref<GoGroup> {
        self.0.borrow()
    }
    pub(crate) fn borrow_mut(&self) -> RefMut<GoGroup> {
        self.0.borrow_mut()
    }
}
impl fmt::Display for GoGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        res.push_str(&self.stone.to_string());
        res.push_str("[");
        for c in self.cells.iter() {
            res.push_str(&c.to_string());
            res.push_str(" ");
        }
        res.push_str("]");

        write!(f, "{}", res)
    }
}

impl fmt::Display for GoGroupRc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.borrow())
    }
}

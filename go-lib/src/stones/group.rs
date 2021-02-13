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

impl fmt::Display for GoGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        res.push_str(&self.stone.to_string());
        write!(f, "{}", res)
    }
}

impl GoGroup {
    pub(crate) fn new(stone: Stone) -> GoGroup {
        GoGroup {
            stone,
            cells: BitSet::with_capacity(GOBAN_SIZE * GOBAN_SIZE),
        }
    }


    pub(crate) fn set_stone(&mut self, stone: Stone) {
        self.stone = stone;
    }

    pub(crate) fn add_cells(&mut self, cells: &[GoCell]) {
        for &c in cells {
            self.cells.insert(c);
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
        self.borrow_mut().add_cells(cells);
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
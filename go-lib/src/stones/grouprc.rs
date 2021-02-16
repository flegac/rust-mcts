use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use bit_set::BitSet;

use board::goban::{Goban, GoCell};
use stones::group::GoGroup;
use stones::stone::Stone;

#[derive(Eq, PartialEq)]
pub struct GoGroupRc(Rc<RefCell<GoGroup>>);

impl GoGroupRc {
    pub(crate) fn from_cell(stone: Stone, cell: GoCell) -> GoGroupRc {
        GoGroupRc::from_group(GoGroup::from_cell(stone, cell))
    }


    pub(crate) fn from_group(group: GoGroup) -> GoGroupRc {
        GoGroupRc(Rc::new(RefCell::new(group)))
    }

    pub(crate) fn new(stone: Stone, cells: BitSet) -> Self {
        GoGroupRc::from_group(GoGroup::new(stone, cells))
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

impl fmt::Display for GoGroupRc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.borrow())
    }
}


impl Hash for GoGroupRc {
    fn hash<H: Hasher>(&self, state: &mut H) {
        return self.borrow().hash(state);
    }
}
use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use board::stones::group::GoGroup;

//FIXME: remove Clone ?
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct GoGroupRc(Rc<RefCell<GoGroup>>);

impl GoGroupRc {
    pub fn from(group: GoGroup) -> GoGroupRc {
        GoGroupRc(Rc::new(RefCell::new(group)))
    }

    pub fn clone(&self) -> Self {
        GoGroupRc(Rc::clone(&self.0))
    }

    pub fn borrow(&self) -> Ref<GoGroup> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<GoGroup> {
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
        self.borrow().hash(state)
    }
}

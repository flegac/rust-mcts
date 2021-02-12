use std::collections::HashSet;

use crate::cell::GoCell;
use crate::stone::Stone;

pub(crate) struct StoneGroup {
    stone: Option<Stone>,
    cells: HashSet<GoCell>,
}

impl StoneGroup {
    pub(crate) fn new(stone: Option<Stone>) -> StoneGroup {
        StoneGroup {
            stone,
            cells: HashSet::new(),
        }
    }
}

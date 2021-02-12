use bit_set::BitSet;

use crate::board::GoCell;
use crate::constants::GOBAN_SIZE;
use crate::stone::Stone;

pub(crate) struct StoneGroup {
    pub(crate) stone: Option<Stone>,
    pub(crate) cells: BitSet,
}


impl StoneGroup {
    pub(crate) fn new(stone: Option<Stone>) -> StoneGroup {
        StoneGroup {
            stone,
            cells: BitSet::with_capacity(GOBAN_SIZE * GOBAN_SIZE),
        }
    }
}

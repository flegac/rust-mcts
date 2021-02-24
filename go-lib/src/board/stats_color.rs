use std::fmt;

use board::go::Go;
use board::goboard::GoBoard;
use stones::stone::Stone;

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) struct ColorStats {
    pub(crate) stone: Stone,
    pub(crate) groups: usize,
    pub(crate) stones: usize,
    pub(crate) captured: usize,
    pub(crate) territory: usize,
}

impl ColorStats {
    pub fn init(stone: Stone) -> ColorStats {
        ColorStats {
            stone,
            stones: 0,
            groups: 0,
            captured: 0,
            territory: 0,
        }
    }
    pub fn new(stone: Stone, board: &GoBoard) -> ColorStats {
        ColorStats {
            stone,
            stones: Go::count_stones(stone, board),
            groups: Go::count_groups(stone, board),
            captured: 0,
            territory: match stone {
                Stone::None => 0,
                _ => Go::count_territory(stone, board)
            },
        }
    }

    pub(crate) fn assert_eq(&self, other: &ColorStats) {
        assert_eq!(self.stones, other.stones, "[{}] stones", self.stone);
        assert_eq!(self.groups, other.groups, "[{}] groups", self.stone);
    }
}


impl fmt::Display for ColorStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} stones, {} groups",
               &self.stone,
               &self.stones,
               &self.groups
        )
    }
}

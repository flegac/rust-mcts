use std::fmt;

use itertools::Itertools;

use board::goboard::GoBoard;
use stones::stone::Stone;

pub(crate) struct StoneStats {
    pub(crate) stone: Stone,
    pub(crate) groups: usize,
    pub(crate) stones: usize,
    pub(crate) captured: usize,
}

impl StoneStats {
    pub(crate) fn new(stone: Stone) -> StoneStats {
        StoneStats {
            stone,
            stones: 0,
            groups: 0,
            captured: 0,
        }
    }
}

pub(crate) struct GoBoardStats {
    pub(crate) black: StoneStats,
    pub(crate) white: StoneStats,
    pub(crate) none: StoneStats,
}

impl GoBoardStats {
    pub(crate) fn new() -> GoBoardStats {
        GoBoardStats {
            black: StoneStats::new(Stone::Black),
            white: StoneStats::new(Stone::White),
            none: StoneStats::new(Stone::None),
        }
    }
}


impl fmt::Display for StoneStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        res.push_str(&self.stone.to_string());
        res.push_str(": ");
        res.push_str(&self.stones.to_string());
        res.push_str(" stones, ");
        res.push_str(&self.groups.to_string());
        res.push_str(" groups");
        write!(f, "{}", res)
    }
}

impl fmt::Display for GoBoardStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        res.push_str(&self.black.to_string());
        res.push_str("\n");
        res.push_str(&self.white.to_string());
        res.push_str("\n");
        res.push_str(&self.none.to_string());
        write!(f, "{}", res)
    }
}
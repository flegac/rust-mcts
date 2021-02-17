use std::fmt;

use itertools::Itertools;

use board::goboard::GoBoard;
use stones::stone::Stone;

pub(crate) struct StoneStats {
    pub(crate) stone: Stone,
    pub(crate) groups: usize,
    pub(crate) stones: usize,
    pub(crate) captured: usize,
    pub(crate) territory: usize,
}

impl StoneStats {
    pub(crate) fn new(stone: Stone) -> StoneStats {
        StoneStats {
            stone,
            stones: 0,
            groups: 0,
            captured: 0,
            territory: 0,
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
        write!(f, "{}", format!("{}: {} tones, {} groups",
                                &self.stone,
                                &self.stones,
                                &self.groups
        ).as_str())
    }
}

impl fmt::Display for GoBoardStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{}\n{}\n{}",
                                self.black,
                                self.white,
                                self.none
        ))
    }
}
use std::fmt;

use board::stats::board_stats::{BoardStats, FullStats};
use board::stats::stone_score::StoneScore;
use board::stats::stone_stats::StoneStats;
use board::stones::stone::Stone;

impl fmt::Display for StoneScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: territories={}, captured={}",
               self.stone,
               self.territory,
               self.captures)
    }
}


impl fmt::Display for StoneStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} stones, {} groups, {} captured",
               &self.stone,
               &self.stones,
               &self.groups,
               &self.captured
        )
    }
}

impl fmt::Display for BoardStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}\n{}",
               self.stats(Stone::Black),
               self.stats(Stone::White),
               self.stats(Stone::None),
        )
    }
}

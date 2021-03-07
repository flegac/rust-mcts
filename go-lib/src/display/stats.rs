use std::fmt;

use board::go_state::GoState;
use board::stats::full_stats::{BoardStats, FullStats};
use board::stats::stone_score::StoneScore;
use board::stats::stone_stats::StoneStats;
use board::stones::stone::Stone;

impl fmt::Display for StoneScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: territory={:2}, captured={:2}",
               self.stone,
               self.territory,
               self.captures)
    }
}


impl fmt::Display for StoneStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:2} stones {:2} groups {:2} captured | territory={:3}",
               self.stone,
               self.stones,
               self.groups,
               self.captured,
               self.territory,
        )
    }
}

impl GoState {
    pub(crate) fn stats_str(&self) -> String {
        let mut black = self.stats(Stone::Black).to_string();
        let mut white = self.stats(Stone::White).to_string();
        let none = self.stats(Stone::None).to_string();
        match self.stone {
            Stone::None => {}
            Stone::Black => {
                black = format!("[{}]", black);
                white = format!(" {} ", white);
            }
            Stone::White => {
                black = format!(" {} ", black);
                white = format!("[{}]", white);
            }
        }
        format!("{}\n{}\n {} ",
                black,
                white,
                none,
        )
    }
}

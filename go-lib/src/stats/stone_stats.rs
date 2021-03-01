use std::fmt;

use board::go::Go;
use board::go_state::GoState;
use display::display::GoDisplay;
use display::templates::GoTemplate;
use rust_tools::screen::layout::layout::L;
use rust_tools::screen::layout::template::Template;
use stones::stone::Stone;
use stats::board_stats::FullStats;

pub struct StoneStats {
    pub stone: Stone,
    pub groups: usize,
    pub stones: usize,
    pub captured: usize,
    pub territory: usize,
}

impl StoneStats {
    pub fn init(stone: Stone) -> StoneStats {
        StoneStats {
            stone,
            stones: 0,
            groups: 0,
            captured: 0,
            territory: 0,
        }
    }

    pub fn new(stone: Stone, board: &GoState) -> StoneStats {
        let go = Go::new(board);
        StoneStats {
            stone,
            stones: go.count_stones(stone),
            groups: go.count_groups(stone),
            captured: board.stats(stone).captured,
            territory: go.count_territory(stone),
        }
    }

    pub fn assert_eq(&self, other: &StoneStats) {
        assert_eq!(self.captured, other.captured, "[{}] captures", self.stone);
        assert_eq!(self.stones, other.stones, "[{}] stones", self.stone);
        assert_eq!(self.groups, other.groups, "[{}] groups", self.stone);
    }
}

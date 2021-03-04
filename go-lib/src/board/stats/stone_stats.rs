use board::stats::board_stats::FullStats;
use board::stones::stone::Stone;
use board::go_state::GoState;
use go_rules::go::Go;

#[derive(Debug,Copy, Clone)]
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
        assert_eq!(self.groups, other.groups, "[{}] stones", self.stone);
    }
}

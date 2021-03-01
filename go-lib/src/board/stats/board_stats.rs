use board::go_state::GoState;
use board::stats::stone_score::StoneScore;
use board::stats::stone_stats::StoneStats;
use board::groups::stone::Stone;
use board::groups::groups1::GoGroup;

pub trait FullStats {
    fn score(&self, stone: Stone) -> StoneScore;
    fn stats(&self, stone: Stone) -> StoneStats;
    fn add_prisoners(&mut self, stone: Stone, n: usize);
    fn set_territory(&mut self, stone: Stone, n: usize);
}

#[derive(Copy, Clone)]
pub struct BoardStats {
    black: StoneStats,
    white: StoneStats,
    none: StoneStats,
    pub round: usize,
}

impl FullStats for BoardStats {
    fn score(&self, stone: Stone) -> StoneScore {
        let x = self.for_stone(stone);
        let y = self.for_stone(stone.switch());
        StoneScore {
            stone: stone,
            territory: x.territory,
            captures: y.captured,
        }
    }

    fn stats(&self, stone: Stone) -> StoneStats {
        let x = self.for_stone(stone);
        StoneStats {
            stone,
            groups: x.groups,
            stones: x.stones,
            captured: x.captured,
            territory: 0,
        }
    }

    fn add_prisoners(&mut self, stone: Stone, n: usize) {
        if stone != Stone::None {
            self.for_stone_mut(stone).captured += n;
        }
    }

    fn set_territory(&mut self, stone: Stone, n: usize) {
        self.for_stone_mut(stone).territory = n;
    }
}

impl BoardStats {
    pub fn assert_eq(&self, other: &BoardStats) {
        self.black.assert_eq(&other.black);
        self.white.assert_eq(&other.white);
        self.none.assert_eq(&other.none);
    }


    pub fn new() -> BoardStats {
        BoardStats {
            black: StoneStats::init(Stone::Black),
            white: StoneStats::init(Stone::White),
            none: StoneStats::init(Stone::None),
            round: 0,
        }
    }

    pub fn from_board(board: &GoState) -> BoardStats {
        BoardStats {
            black: StoneStats::new(Stone::Black, board),
            white: StoneStats::new(Stone::White, board),
            none: StoneStats::new(Stone::None, board),
            round: board.stats.round,
        }
    }


    #[inline]
    pub fn for_stone(&self, stone: Stone) -> &StoneStats {
        match stone {
            Stone::Black => &self.black,
            Stone::White => &self.white,
            Stone::None => &self.none
        }
    }

    #[inline]
    pub fn for_stone_mut(&mut self, stone: Stone) -> &mut StoneStats {
        match stone {
            Stone::Black => &mut self.black,
            Stone::White => &mut self.white,
            Stone::None => &mut self.none
        }
    }

    pub fn add_group(&mut self, group: &GoGroup) {
        let n = group.stones();
        match group.stone {
            Stone::Black => {
                self.black.groups += 1;
                self.black.stones += n;
                self.none.stones -= n;
            }
            Stone::White => {
                self.white.groups += 1;
                self.white.stones += n;
                self.none.stones -= n;
            }
            Stone::None => {
                self.none.groups += 1;
                self.none.stones += n;
            }
        }
    }

    pub fn rem_group(&mut self, group: &GoGroup) {
        let n = group.stones();
        match group.stone {
            Stone::Black => {
                self.black.groups -= 1;
                self.black.stones -= n;
                self.none.stones += n;
            }
            Stone::White => {
                self.white.groups -= 1;
                self.white.stones -= n;
                self.none.stones += n;
            }
            Stone::None => {
                self.none.groups -= 1;
                self.none.stones -= n;
            }
        }
    }
}

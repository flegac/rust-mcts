use std::fmt;

use board::goboard::GoBoard;
use stones::group::GoGroup;
use stones::stone::Stone;

use crate::board::stats_color::ColorStats;

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) struct BoardStats {
    pub(crate) black: ColorStats,
    pub(crate) white: ColorStats,
    pub(crate) none: ColorStats,
    pub(crate) round: usize,
}

impl BoardStats {
    pub(crate) fn assert_eq(&self, other: &BoardStats) {
        self.black.assert_eq(&other.black);
        self.white.assert_eq(&other.white);
        self.none.assert_eq(&other.none);
    }


    pub(crate) fn new() -> BoardStats {
        BoardStats {
            black: ColorStats::init(Stone::Black),
            white: ColorStats::init(Stone::White),
            none: ColorStats::init(Stone::None),
            round: 0,
        }
    }

    pub fn from_board(board: &GoBoard) -> BoardStats {
        BoardStats {
            black: ColorStats::new(Stone::Black, board),
            white: ColorStats::new(Stone::White, board),
            none: ColorStats::new(Stone::None, board),
            round: board.stats.round,
        }
    }


    #[inline]
    pub fn for_stone(&self, stone: Stone) -> ColorStats {
        match stone {
            Stone::Black => self.black,
            Stone::White => self.white,
            Stone::None => self.none
        }
    }

    pub fn compute_round(&self) -> usize {
        let blacks = self.black.stones + self.black.captured;
        let whites = self.white.stones + self.white.captured;
        blacks + whites
    }

    pub(crate) fn add_group(&mut self, group: &GoGroup) {
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
        log::trace!("add {}: {}\n{}", n, group, self);
    }

    pub(crate) fn rem_group(&mut self, group: &GoGroup) {
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
        log::trace!("rem {}: {}\n{}", n, group, self);
    }


    pub(crate) fn score_string(&self) -> String {
        format!("\
            black: territories={}, captured={}\n\
            white: territories={}, captured={}",
                self.black.territory,
                self.black.captured,
                self.white.territory,
                self.white.captured
        )
    }
}

impl fmt::Display for BoardStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}\n{}",
               self.black,
               self.white,
               self.none
        )
    }
}
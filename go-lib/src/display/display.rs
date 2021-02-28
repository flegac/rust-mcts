use std::borrow::Borrow;
use std::fmt;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};
use std::ptr::write;

use bit_set::BitSet;
use graph_lib::topology::Topology;
use itertools::Itertools;

use action::GoAction;
use board::go_state::GoState;
use display::goshow::GoShow;
use display::range::Range2;
use groups::group_access::GroupAccess;
use rust_tools::screen::drawer::Drawer;
use rust_tools::screen::layout::hlayout::HLayout;
use rust_tools::screen::layout::layout::{L, LayoutRc};
use rust_tools::screen::layout::template::Template;
use rust_tools::screen::layout::vlayout::VLayout;
use rust_tools::screen::screen::Screen;
use sgf::sgf_export::{Sequence, SGF};
use stats::board_stats::{BoardStats, FullStats};
use stats::stone_score::StoneScore;
use stats::stone_stats::StoneStats;
use stones::group::GoGroup;
use stones::stone::Stone;

pub struct GoDisplay {}

const BIG_A: usize = 'A' as usize;
const SMALL_A: usize = 'a' as usize;


impl fmt::Display for GoState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\nhistory({}):\n{}\n",
               GoDisplay::board(&self).to_string(),
               self.stats.round,
               GoDisplay::game(&self)
        )
    }
}

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

impl GoState {
    pub(crate) fn score_str(&self) -> String {
        let mut blacks = self.score(Stone::Black).to_string();
        let mut whites = self.score(Stone::White).to_string();

        match self.stone {
            Stone::None => {}
            Stone::Black => blacks = format!("[{}]", blacks),
            Stone::White => whites = format!("[{}]", whites)
        }

        format!("{}\n{}", blacks, whites)
    }
}


impl GoDisplay {
    pub fn board_str(board: &GoState, range: Range2) -> String {
        let mut res = String::new();
        let columns = String::from_iter(
            range.x.iter()
                .map(Self::column)
                .map(|x| format!(" {} ", x))
        );
        let separator = String::from_iter(range.x.iter().map(|x| "---"));
        res.push_str(&format!("  +{}+\n", separator));
        for y in range.y.iter().rev() {
            res.push_str(&format!("{} |", GoDisplay::line(y)));
            for x in range.x.iter() {
                let stone = Self::stone(board.stone_at(board.goban().cell(x, y)));
                res.push_str(&format!(" {} ", stone));
            }
            res.push_str(&format!("|\n"));
        }
        res.push_str(&format!("  +{}+\n", separator));
        res.push_str(&format!("   {}\n", columns));

        res
    }
}


impl GoShow for GoDisplay {
    fn game(board: &GoState) -> Sequence {
        SGF::game(board.goban().size, Stone::Black, board.history.as_slice())
    }

    fn board(board: &GoState) -> LayoutRc {
        let range = Range2::board(board.goban().size);
        L::str(&format!("{}\n{}\n{}",
                        Self::board_str(board, range),
                        board.score_str(),
                        board.stats
        ))
    }

    fn board_range(board: &GoState, range: Range2) -> LayoutRc {
        L::str(&Self::board_str(board, range))
    }
    fn group_layout(board: &GoState, group: &GoGroup) -> LayoutRc {
        let range = group.cells.iter()
            .map(|c| board.goban().xy(c))
            .fold(Range2::empty(), |c, v| c.merge(v));
        Self::board_range(board, range)
    }

    fn group(board: &GoState, group: &GoGroup) -> String {
        let mut res = String::new();
        res.push_str("{");
        res.push_str(&format!("{} #{}:", group.stone, group.stones()));
        for cell in group.cells.iter() {
            res.push_str(" ");
            res.push_str(&GoDisplay::cell(board.goban().xy(cell)));
        }
        res.push_str("}");
        res
    }

    fn cell(xy: (usize, usize)) -> String {
        format!("{}{}", Self::column(xy.0), Self::line(xy.1))
    }

    fn cells(board: &GoState, stone: Stone, cells: &BitSet) -> String {
        let mut res = String::new();
        res.push_str("{");
        res.push_str(&format!("{} #{}:", stone, cells.len()));
        for cell in cells.iter() {
            res.push_str(" ");
            res.push_str(&GoDisplay::cell(board.goban().xy(cell)));
        }
        res.push_str("}");
        res
    }

    fn stone(stone: Stone) -> String {
        format!("{}", stone)
    }

    fn action(action: &GoAction) -> String {
        match action {
            GoAction::Pass => format!("Pass"),
            GoAction::Cell(x, y) => format!("{}", GoDisplay::cell((*x, *y)))
        }
    }

    fn column(x: usize) -> String {
        format!("{}", char::from((x + BIG_A) as u8))
    }

    fn line(y: usize) -> String {
        format!("{}", char::from((y + SMALL_A) as u8))
    }
}
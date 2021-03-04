use std::fmt;
use std::fmt::Display;
use std::iter::FromIterator;

use bit_set::BitSet;
use itertools::Itertools;

use board::action::GoAction;
use board::go_state::GoState;
use board::grid::Grid;
use board::group_access::GroupAccess;
use board::stats::board_stats::{BoardStats, FullStats};
use board::stats::stone_score::StoneScore;
use board::stats::stone_stats::StoneStats;
use board::stones::group::GoGroup;
use board::stones::grouprc::GoGroupRc;
use board::stones::stone::Stone;
use display::goshow::GoShow;
use display::range::Range2;
use graph_lib::topology::Topology;
use rust_tools::screen::layout::layout::{L, LayoutRc};
use sgf::sgf_export::{Sequence, SGF};

use crate::display::board_map::BoardMap;

pub struct GoDisplay {}

const BIG_A: usize = 'A' as usize;
const SMALL_A: usize = 'a' as usize;


impl fmt::Display for GoState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}\n",
               GoDisplay::board(&self).to_string(),
               self.stats.round
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

fn empty_group_id(g: GoGroupRc) -> Option<String> {
    Some(match g.borrow().stone {
        // Stone::None => {
        //     format!(" [{:3}]", g.borrow().id)
        // }
        s => {
            format!("    {} ", GoDisplay::stone(s))
        }
    })
}

impl GoDisplay {
    pub fn history_str(board: &GoState, range: Range2) -> String {
        let mut hist = BoardMap {
            width: board.goban().size,
            height: board.goban().size,
            map: vec![None; board.vertex_number()],
        };
        for (i, a) in board.history.iter().enumerate() {
            match a {
                GoAction::Pass => {}
                GoAction::Cell(x, y) => {
                    let cell = board.goban().cell(*x, *y);
                    hist.map[cell] = Some(i);
                }
            }
        }
        hist.map_str(range, 3)
    }
}


impl GoShow for GoDisplay {
    fn sgf(board: &GoState) -> Sequence {
        SGF::game(board.goban().size, Stone::Black, board.history.as_slice())
    }

    fn history(board: &GoState) -> LayoutRc {
        let range = Range2::board(board.goban().size);
        L::str(&Self::history_str(board, range))
    }

    fn board(board: &GoState) -> LayoutRc {
        let range = Range2::board(board.goban().size);
        L::vert(vec![
            Self::board_range(board, range),
            L::str(&board.score_str()),
            L::str(&board.stats.to_string())
        ])
    }

    fn board_range(board: &GoState, range: Range2) -> LayoutRc {
        let map = BoardMap::new(board)
            .map(|g| empty_group_id(g.clone()));
        L::str(&map.map_str(range, 6))
    }

    fn group_layout(board: &GoState, group: &GoGroupRc) -> LayoutRc {
        let range = group.borrow().cells.iter()
            .map(|c| board.goban().xy(c))
            .fold(Range2::empty(), |c, v| c.merge(v));
        Self::board_range(board, range)
    }

    fn group(board: &GoState, group: &GoGroup) -> String {
        let mut res = String::new();
        res.push_str("{");
        res.push_str(&format!("{}: {} [{}]", group.id, group.stones(), group.stone, ));
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
        res.push_str(&format!("{{{} [{}]", cells.len(), stone));
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
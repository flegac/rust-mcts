use std::fmt;
use std::fmt::Display;
use std::iter::FromIterator;

use bit_set::BitSet;
use itertools::Itertools;

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
use go_rules::go_action::GoAction;
use graph_lib::topology::Topology;
use rust_tools::screen::layout::layout::{L, LayoutRc};
use sgf::sgf_export::{Sequence, SGF};

use crate::board::go_state::GoState;
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
        let mut blacks = self.stats.score(Stone::Black).to_string();
        let mut whites = self.stats.score(Stone::White).to_string();

        match self.stone {
            Stone::None => {}
            Stone::Black => blacks = format!("[{}]", blacks),
            Stone::White => whites = format!("[{}]", whites)
        }

        format!("{}\n{}", blacks, whites)
    }
}

fn stone_str(g: GoGroupRc) -> Option<String> {
    let stone = g.borrow().stone;
    Some(format!(" {}", GoDisplay::stone(stone)))
}

fn group_id(g: GoGroupRc) -> Option<String> {
    Some(match g.borrow().stone {
        s => {
            format!(" [{:3}]", g.borrow().id)
        }
    })
}

impl GoDisplay {
    pub fn history_str(board: &GoState, range: &Range2) -> String {
        let goban = board.gg.goban();
        let mut hist = BoardMap {
            width: goban.size,
            height: goban.size,
            map: vec![None; goban.vertex_number()],
        };
        for (i, a) in board.history.iter().enumerate() {
            match a {
                GoAction::Pass => {}
                GoAction::Cell(x, y) => {
                    let cell = board.gg.goban().cell(*x, *y);
                    hist.map[cell] = Some(i);
                }
            }
        }
        hist.map_str(range, 3)
    }
}


impl GoShow for GoDisplay {
    fn sgf(board: &GoState) -> Sequence {
        SGF::game(board.gg.goban().size, Stone::Black, board.history.as_slice())
    }

    fn history(board: &GoState) -> LayoutRc {
        let range = Range2::board(board.gg.goban().size);
        L::str(&Self::history_str(board, &range))
    }

    fn board(board: &GoState) -> LayoutRc {
        let range = Range2::board(board.gg.goban().size);
        L::vert(vec![
            Self::board_range(board, range),
            L::str(&board.score_str()),
            L::str(&board.stats.to_string())
        ])
    }

    fn board_range(board: &GoState, range: Range2) -> LayoutRc {
        let classic = BoardMap::new(board)
            .map(|g| stone_str(g.clone()));


        let group_ids = BoardMap::new(board)
            .map(|g| group_id(g.clone()));

        L::vert(vec![
            L::hori(vec![
                L::str(&classic.map_str(&range, 3)),
                L::str(&Self::history_str(board, &range))
            ]),
            L::str(&group_ids.map_str(&range, 6))
        ])
    }

    fn group_layout(board: &GoState, group: &GoGroupRc) -> LayoutRc {
        let range = group.borrow().cells.iter()
            .map(|c| board.gg.goban().xy(c))
            .fold(Range2::empty(), |c, v| c.merge(v));
        Self::board_range(board, range)
    }

    fn group(board: &GoState, group: &GoGroup) -> String {
        let mut res = String::new();
        res.push_str("{");
        res.push_str(&format!("{}: {} [{}]", group.id, group.stones(), group.stone, ));
        for cell in group.cells.iter() {
            res.push_str(" ");
            res.push_str(&GoDisplay::cell(board.gg.goban().xy(cell)));
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
            res.push_str(&GoDisplay::cell(board.gg.goban().xy(cell)));
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
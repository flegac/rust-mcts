use std::fmt;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};

use bit_set::BitSet;
use graph_lib::topology::Topology;
use itertools::Itertools;

use action::GoAction;
use board::goboard::GoBoard;
use board::stats_board::BoardStats;
use board::stats_color::ColorStats;
use display::goshow::GoShow;
use display::range::Range2;
use rust_tools::screen::drawer::Drawer;
use rust_tools::screen::layout::hlayout::HLayout;
use rust_tools::screen::layout::layout::{L, Layout, LayoutRc};
use rust_tools::screen::layout::template::Template;
use rust_tools::screen::layout::vlayout::VLayout;
use rust_tools::screen::screen::Screen;
use stones::group::GoGroup;
use stones::stone::Stone;

pub struct GoDisplay {}

const BIG_A: usize = 'A' as usize;
const SMALL_A: usize = 'a' as usize;


impl fmt::Display for ColorStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} stones, {} groups, {} captured",
               &self.stone,
               &self.stones,
               &self.groups,
               &self.captured
        )
    }
}

struct GoTemplate {}

// impl GoTemplate {
//     pub fn color_stats(&self) -> Template {
//         let stone = L::str("");
//         let stones = L::str("");
//         let groups = L::str("");
//         let captured = L::str("");
//         let mut res = Template::new(L::hori(vec![
//             stone.clone(),
//             L::str(": "),
//             stones.clone(),
//             L::str(" stones, "),
//             groups.clone(),
//             L::str(" groups, "),
//             captured.clone(),
//             L::str(" captured"),
//         ]));
//         res.register(0, &stone);
//         res.register(1, &stones);
//         res.register(2, &groups);
//         res.register(3, &captured);
//         res
//     }
//
//     pub fn stats_template(&self) -> Template {
//         let black = self.color_stats();
//         let white = self.color_stats();
//         let none = self.color_stats();
//
//         let mut res = Template::new(L::vert(vec![
//             black.template,
//             white.template,
//             none.template,
//         ]));
//         let mut i = 0;
//         for (k, v) in black.vars {
//             res.register(k, &v);
//         }
//         for (k, v) in white.vars {
//             res.register(k, &v);
//         }
//         for (k, v) in none.vars {
//             res.register(k, &v);
//         }
//
//         res
//     }
// }

impl GoBoard {
    pub(crate) fn stats_str(&self) -> String {
        format!("{}\n{}\n{}",
                self.stats.black.to_string(),
                self.stats.white.to_string(),
                self.stats.none.to_string(),
        )
    }

    pub(crate) fn score_str(&self) -> String {
        let mut blacks = format!("black: territories={}, captured={}",
                                 self.stats.black.territory,
                                 self.stats.black.captured);
        let mut whites = format!("white: territories={}, captured={}",
                                 self.stats.white.territory,
                                 self.stats.white.captured);
        match self.stone {
            Stone::None => {}
            Stone::Black => blacks = format!("[{}]", blacks),
            Stone::White => whites = format!("[{}]", whites)
        }

        format!("{}\n{}", blacks, whites)
    }
}


impl GoDisplay {
    pub fn board_str(board: &GoBoard, range: Range2) -> String {
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
                let stone = Self::stone(board.stone_at(board.goban.cell(x, y)));
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
    fn board(board: &GoBoard) -> LayoutRc {
        let range = Range2::board(board.goban.size);
        L::str(&format!("{}\n{}\n{}",
                        Self::board_str(board, range),
                        board.score_str(),
                        board.stats_str()
        ))
    }

    fn board_range(board: &GoBoard, range: Range2) -> LayoutRc {
        L::str(&Self::board_str(board, range))
    }
    fn group_layout(board: &GoBoard, group: &GoGroup) -> LayoutRc {
        let range = group.cells.iter()
            .map(|c| board.goban.xy(c))
            .fold(Range2::empty(), |c, v| c.merge(v));
        Self::board_range(board, range)
    }

    fn group(board: &GoBoard, group: &GoGroup) -> String {
        let mut res = String::new();
        res.push_str("{");
        res.push_str(&format!("{} #{}:", group.stone, group.stones()));
        for cell in group.cells.iter() {
            res.push_str(" ");
            res.push_str(&GoDisplay::cell(board.goban.xy(cell)));
        }
        res.push_str("}");
        res
    }

    fn cell(xy: (usize, usize)) -> String {
        format!("{}{}", Self::column(xy.0), Self::line(xy.1))
    }

    fn cells(board: &GoBoard, stone: Stone, cells: &BitSet) -> String {
        let mut res = String::new();
        res.push_str("{");
        res.push_str(&format!("{} #{}:", stone, cells.len()));
        for cell in cells.iter() {
            res.push_str(" ");
            res.push_str(&GoDisplay::cell(board.goban.xy(cell)));
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
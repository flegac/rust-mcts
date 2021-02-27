use std::fmt;
use std::iter::FromIterator;

use bit_set::BitSet;
use graph_lib::topology::Topology;
use itertools::Itertools;

use action::GoAction;
use board::goboard::GoBoard;
use board::stats_board::BoardStats;
use display::goshow::GoShow;
use rust_tools::screen::dimension::{Cursor, Dimension, ScreenIndex};
use rust_tools::screen::drawer::Drawer;
use rust_tools::screen::layout::hlayout::HLayout;
use rust_tools::screen::layout::layout::{L, Layout2, LayoutRc};
use rust_tools::screen::layout::str_layout::StrLayout;
use rust_tools::screen::layout::vlayout::VLayout;
use rust_tools::screen::layout_old::Layout;
use rust_tools::screen::screen::Screen;
use stones::group::GoGroup;
use stones::stone::Stone;

pub struct GoDisplay {}

const BIG_A: usize = 'A' as usize;
const SMALL_A: usize = 'a' as usize;

impl BoardStats {
    pub(crate) fn stats_layout(&self) -> LayoutRc {
        L::vert(
            vec![
                L::str(&self.black.to_string()),
                L::str(&self.white.to_string()),
                L::str(&self.none.to_string()),
            ]
        )
    }

    pub(crate) fn score_layout(&self, stone: Stone) -> LayoutRc {
        let mut blacks = format!("black: territories={}, captured={}",
                                 self.black.territory,
                                 self.black.captured);
        let mut whites = format!("white: territories={}, captured={}",
                                 self.white.territory,
                                 self.white.captured);
        match stone {
            Stone::None => {}
            Stone::Black => blacks = format!("[{}]", blacks),
            Stone::White => whites = format!("[{}]", whites)
        }

        L::vert(
            vec![
                L::str(&blacks),
                L::str(&whites),
            ]
        )
    }
}


// fn with_notation(&self, sparse: bool) -> Self {
//     // compute notation
//     let mut columns = Screen::from_string(&String::from_iter((0..self.width())
//         .map(GoDisplay::column)));
//     let mut lines = Screen::from_string(&String::from_iter((0..self.width())
//         .map(GoDisplay::line)));
//     lines.transpose();
//
//     let mut full = match sparse {
//         true => {
//             columns = columns.sparse();
//             self.sparse().border().grow(1)
//         }
//         false => {
//             self.border().grow(1)
//         }
//     };
//
//     full.draw_at(full.index(2, 0), &columns);
//     full.draw_at(full.index(2, -1), &columns);
//     full.draw_at(full.index(0, 2), &lines);
//     full.draw_at(full.index(-1, 2), &lines);
//
//     full
// }

impl GoDisplay {
    pub fn board_layout(board: &GoBoard) -> LayoutRc {
        let mut res = vec![];
        for y in 0..board.goban.size {
            res.push(Self::line_layout(board, y));
        }
        // res.push(board.stats.score_layout(board.stone));


        L::vert(vec![
            L::vert(res),
            board.stats.score_layout(board.stone),
            board.stats.stats_layout()
        ])

        // L::vert((0..board.goban.size)
        //     .map(|y| Self::line_layout(board, y))
        //     .collect_vec()
        // )
    }

    pub fn line_layout(board: &GoBoard, y: usize) -> LayoutRc {
        let mut res = vec![];
        for x in 0..board.goban.size {
            let cell = board.goban.cell(x, y);
            let stone = Self::stone(board.stone_at(cell));
            let l = L::str(&format!(" {} ", stone.as_str()));
            res.push(l);
        }
        L::hori(res)
    }
}


impl GoShow for GoDisplay {
    fn board(board: &GoBoard) -> Screen {
        // let size = board.goban.size;
        // let mut screen = Screen::new(size, size);
        // for c in board.vertices() {
        //     let value = Self::stone(board.stone_at(c));
        //     screen.put(c, value.chars().next().unwrap());
        // }
        // screen
        //     .with_notation(true).with_stats(board, true, true)

        Self::board_layout(board).as_screen()
        // .with_notation(true)
        // .with_stats(board, true, true)
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

    fn cells(board: &GoBoard, cells: &BitSet) -> String {
        let mut res = String::new();
        res.push_str("{");
        res.push_str(&format!("#{}:", cells.len()));
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


#[cfg(test)]
mod tests {
    use rust_tools::screen::screen::Screen;

    #[test]
    fn test_with_annotation() {
        let scr = Screen::new(7, 3);

        println!("{}", scr);
    }
}
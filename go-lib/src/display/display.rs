use std::fmt;
use std::iter::FromIterator;

use bit_set::BitSet;
use graph_lib::topology::Topology;

use action::GoAction;
use board::goboard::GoBoard;
use board::stats_board::BoardStats;
use display::goshow::GoShow;
use rust_tools::screen::dimension::{Cursor, Dimension, ScreenIndex};
use rust_tools::screen::drawer::Drawer;
use rust_tools::screen::layout::hlayout::HLayout;
use rust_tools::screen::layout::layout::{L, Layout2};
use rust_tools::screen::layout::str_layout::StrLayout;
use rust_tools::screen::layout::vlayout::VLayout;
use rust_tools::screen::screen::Screen;
use stones::group::GoGroup;
use stones::stone::Stone;
use rust_tools::screen::layout_old::Layout;

pub struct GoDisplay {}

const BIG_A: usize = 'A' as usize;
const SMALL_A: usize = 'a' as usize;

// impl fmt::Display for GoBoard {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}{}\n{}",
//                self.draw_board(),
//                self.stats.score_string(),
//                self.stats
//         )
//     }
// }

impl BoardStats {
    pub(crate) fn stats_screen(&self) -> Screen {
        let blacks = self.black.to_string();
        let whites = self.white.to_string();
        let nones = self.none.to_string();

        let width = blacks.len().max(whites.len()).max(nones.len());
        let mut scr = Screen::new(width, 3);
        scr.put_str(scr.index(0, 0), &blacks);
        scr.put_str(scr.index(0, 1), &whites);
        scr.put_str(scr.index(0, 2), &nones);
        scr
    }

    pub(crate) fn score_screen(&self, stone: Stone) -> Screen {
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


        let width = blacks.len().max(whites.len());
        let mut scr = Screen::new(width, 2);
        scr.put_str(scr.index(0, 0), &blacks);
        scr.put_str(scr.index(0, 1), &whites);
        scr
    }
}


trait GoScreen {
    fn with_notation(&self, sparse: bool) -> Self;
    fn with_stats(&self, board: &GoBoard, score: bool, stats: bool) -> Self;
}


impl GoScreen for Screen {
    fn with_notation(&self, sparse: bool) -> Self {
        // compute notation
        let mut columns = Screen::from_string(&String::from_iter((0..self.width())
            .map(GoDisplay::column)));
        let mut lines = Screen::from_string(&String::from_iter((0..self.width())
            .map(GoDisplay::line)));
        lines.transpose();

        let mut full = match sparse {
            true => {
                columns = columns.sparse();
                self.sparse().border().grow(1)
            }
            false => {
                self.border().grow(1)
            }
        };

        full.draw_at(full.index(2, 0), &columns);
        full.draw_at(full.index(2, -1), &columns);
        full.draw_at(full.index(0, 2), &lines);
        full.draw_at(full.index(-1, 2), &lines);

        full
    }

    fn with_stats(&self, board: &GoBoard, score: bool, stats: bool) -> Self {
        let x = board.stats.score_screen(board.stone);
        let y = board.stats.stats_screen();
        let mut screens = vec![self];
        if score {
            screens.push(&x);
        }
        if stats {
            screens.push(&y);
        }
        Screen::valign(screens.as_slice(), 0)
    }
}

impl GoDisplay {
    pub fn board_layout(board: &GoBoard) -> HLayout<VLayout<StrLayout>> {
        let mut res = vec![];
        for y in 0..board.goban.size {
            res.push(Self::line_layout(board, y));
        }
        L::hori(res)
    }

    pub fn line_layout(board: &GoBoard, y: usize) -> VLayout<StrLayout> {
        let mut res = vec![];
        for x in 0..board.goban.size {
            let cell = board.goban.cell(x, y);
            let stone = Self::stone(board.stone_at(cell));
            let l = L::str(stone.as_str());
            res.push(l);
        }
        L::vert(res)
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
            .with_notation(true).with_stats(board, true, true)
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
    use display::display::GoScreen;
    use rust_tools::screen::screen::Screen;
    use screen::screen::Screen;

    #[test]
    fn test_with_annotation() {
        let scr = Screen::new(7, 3).with_notation();

        println!("{}", scr);
    }
}
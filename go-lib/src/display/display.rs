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
use rust_tools::screen::screen::Screen;
use stones::group::GoGroup;
use stones::stone::Stone;

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
    fn score_screen(&self) -> Screen {
        let mut scr = Screen::new(35, 2);
        scr.put_str(scr.index(0, 0),
                    &format!("black: territories={}, captured={}",
                             self.black.territory,
                             self.black.captured));
        scr.put_str(scr.index(0, 1),
                    &format!("white: territories={}, captured={}",
                             self.white.territory,
                             self.white.captured));
        scr
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


trait GoScreen {
    fn with_notation(&self, sparse: bool) -> Self;
    fn with_score(&self, board: &GoBoard) -> Self;
}


impl GoScreen for Screen {
    fn with_notation(&self, sparse: bool) -> Self {
        let res = match sparse {
            true => self.sparse().border(),
            false => self.border()
        };
        let mut full = res.grow(1);

        let columns = Screen::from_string(&String::from_iter((0..self.width())
            .map(GoDisplay::column))).sparse();
        full.draw_at(full.index(2, 0), &columns);
        full.draw_at(full.index(2, -1), &columns);

        let mut lines = Screen::from_string(&String::from_iter((0..self.width())
            .map(GoDisplay::line)));
        lines.mirror();
        full.draw_at(full.index(0, 2), &lines);
        full.draw_at(full.index(-1, 2), &lines);

        full
    }

    fn with_score(&self, board: &GoBoard) -> Self {
        let mut full = Screen::new(self.width(), self.height() + 2);
        let score = board.stats.score_screen();
        full.draw(self);
        full.draw_at(full.index(0_i32, -2), &score);
        full
    }
}


impl GoShow for GoDisplay {
    fn board(board: &GoBoard) -> Screen {
        let size = board.goban.size;
        let mut screen = Screen::new(size, size);
        for c in board.vertices() {
            let value = Self::stone(board.stone_at(c));
            screen.put(c, value.chars().next().unwrap());
        }
        screen.with_notation(true).with_score(board)
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
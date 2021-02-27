use std::fmt;

use bit_set::BitSet;
use graph_lib::topology::Topology;

use action::GoAction;
use board::goboard::GoBoard;
use board::stats_board::BoardStats;
use display::goshow::GoShow;
use screen::dimension::{Dimension, ScreenIndex};
use screen::drawer::Drawer;
use screen::screen::Screen;
use stones::group::GoGroup;
use stones::stone::Stone;

const BIG_A: usize = 'A' as usize;
const SMALL_A: usize = 'a' as usize;

pub struct GoDisplay {}

impl GoBoard {
    fn draw_board(&self) -> String {
        GoDisplay::board(self).to_string()
    }
}


impl fmt::Display for GoBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}\n{}",
               self.draw_board(),
               self.stats.score_string(),
               self.stats
        )
    }
}

impl BoardStats {
    fn score_string(&self) -> String {
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

impl Screen {
    fn with_notation(&self) -> Screen {
        let mut full = self.border().grow(1);
        for i in 0..self.width() {
            let x_str = GoDisplay::column(i);
            full.put_str(full.index(i + 2, 0), &x_str);
            full.put_str(full.index((i + 2) as i32, -1), &x_str);
        }
        for i in 0..self.height() {
            let y_str = GoDisplay::line(i);
            full.put_str(full.index(0, i + 2), &y_str);
            full.put_str(full.index(-1, (i + 2) as i32), &y_str);
        }

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
        screen.with_notation()
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
    use screen::screen::Screen;

    #[test]
    fn test_with_annotation() {
        let scr = Screen::new(7, 3).with_notation();

        println!("{}", scr);
    }
}
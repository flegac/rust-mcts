use std::borrow::{Borrow, BorrowMut};
use std::fmt;
use std::fmt::{Display, Formatter};

use bit_set::BitSet;
use itertools::Itertools;

use action::GoAction;
use board::goboard::GoBoard;
use graph_lib::topology::Topology;
use screen::Screen;
use stones::group::GoGroup;
use stones::stone::Stone;

pub struct GoDisplay {
    board: GoBoard
}

const BIG_A: usize = 'A' as usize;
const SMALL_A: usize = 'a' as usize;


impl GoDisplay {
    pub fn cells(board: &GoBoard, cells: &BitSet) -> String {
        let mut res = String::new();
        res.push_str("{");
        res.push_str(&format!("#{}:", cells.len()));
        for (i, cell) in cells.iter().enumerate() {
            res.push_str(" ");
            res.push_str(&GoDisplay::from_cell(board.goban.xy(cell)));
        }
        res.push_str("}");
        res
    }

    pub fn group(board: &GoBoard, group: &GoGroup) -> String {
        let mut res = String::new();
        res.push_str("{");
        res.push_str(&format!("{} #{}:", group.stone, group.stones()));
        for (i, cell) in group.cells.iter().enumerate() {
            res.push_str(" ");
            res.push_str(&GoDisplay::from_cell(board.goban.xy(cell)));
        }
        res.push_str("}");
        res
    }

    pub fn stone_name(stone: Stone) -> String {
        format!("{:?}", stone)
    }

    pub fn from_cell(xy: (usize, usize)) -> String {
        format!("{}{}", Self::from_x(xy.0), Self::from_y(xy.1))
    }

    pub fn action(action: &GoAction) -> String {
        match action {
            GoAction::Pass => format!("Pass"),
            GoAction::Cell(x, y) => format!("{}", GoDisplay::from_cell((*x, *y)))
        }
    }
    pub fn from_x(x: usize) -> String {
        format!("{}", char::from((x + BIG_A) as u8))
    }

    pub fn from_y(y: usize) -> String {
        format!("{}", char::from((y + SMALL_A) as u8))
    }
}

impl fmt::Display for Stone {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Stone::Black => "X",
            Stone::White => "O",
            Stone::None => "."
        })
    }
}


impl fmt::Display for GoGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        res.push_str(&format!("{} #{}:", self.stone, self.stones()));
        for (i, cell) in self.cells.iter().enumerate() {
            res.push_str(" ");
            res.push_str(format!("{} ", cell).as_str());
        }
        res.push_str("}");
        write!(f, "{}", res)
    }
}


impl GoBoard {
    pub fn screen(&self, border: bool) -> Screen {
        let size = self.goban.size;
        let mut screen = Screen::new(size, size);
        for c in self.vertices() {
            let value = match self.stone_at(c) {
                Stone::None => '.',
                Stone::Black => 'X',
                Stone::White => 'O',
            };
            screen.buffer[c] = value;
        }
        let board = Screen::border(&screen);
        match border {
            true => self.with_notation(&board),
            false => board
        }
    }

    fn with_notation(&self, board: &Screen) -> Screen {
        let mut full = Screen::grow(board, 1);
        for i in 0..(board.width - 2) as i32 {
            let x_str = GoDisplay::from_x(i as usize);
            let y_str = GoDisplay::from_y(i as usize);
            full.put_str(i + 2, 0, &x_str);
            full.put_str(i + 2, -1, &x_str);
            full.put_str(0, i + 2, &y_str);
            full.put_str(-1, i + 2, &y_str);
        }
        full
    }

    fn draw_board(&self) -> String {
        self.screen(true).to_string()
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


#[test]
fn test_screen() {
    let mut scr = Screen::new(20, 20);
    scr.draw(2, 2, &Screen::fill('x', 2, 5));

    println!("{}", scr);
}
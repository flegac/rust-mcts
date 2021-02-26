use std::fmt;
use std::fmt::Formatter;

use bit_set::BitSet;

use action::GoAction;
use board::goboard::GoBoard;
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

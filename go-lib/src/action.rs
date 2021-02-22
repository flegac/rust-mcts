use std::fmt;
use std::fmt::Formatter;

use board::grid::{GoCell, Grid};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GoAction {
    Pass,
    Cell(usize, usize),
}


impl GoAction {
    pub fn x(&self) -> Option<&usize> {
        match self {
            GoAction::Pass => { None }
            GoAction::Cell(x, _y) => Some(x)
        }
    }

    pub fn y(self) -> Option<usize> {
        match self {
            GoAction::Pass => None,
            GoAction::Cell(_x, y) => Some(y)
        }
    }

    pub fn cell(&self, goban: &Grid) -> Option<GoCell> {
        match self {
            GoAction::Pass => {
                None
            }
            GoAction::Cell(x, y) => {
                Some(goban.cell(*x, *y))
            }
        }
    }
}

impl fmt::Display for GoAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GoAction::Pass => write!(f, "Pass"),
            GoAction::Cell(x, y) => write!(f, "({},{})", x, y)
        }
    }
}
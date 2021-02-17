use std::fmt;
use std::fmt::Formatter;

use board::goban::GoCell;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GoAction {
    pub cell: Option<GoCell>
}


impl GoAction {
    pub(crate) fn at(cell: GoCell) -> GoAction {
        GoAction { cell: Some(cell) }
    }

    fn pass() -> GoAction {
        GoAction { cell: None }
    }
}


impl fmt::Display for GoAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.cell {
            None => write!(f, "Pass"),
            Some(cell) => write!(f, "{}", cell.to_string())
        }
    }
}
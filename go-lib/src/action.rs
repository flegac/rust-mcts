use std::fmt;
use std::fmt::Formatter;

use board::goban::GoCell;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GoAction {
    pub cell: Option<GoCell>
}


impl GoAction {
    pub(crate) fn play_at(cell: GoCell) -> GoAction {
        GoAction { cell: Some(cell) }
    }

    fn pass() -> GoAction {
        GoAction { cell: None }
    }
}


impl fmt::Display for GoAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        match self.cell {
            None => { res.push_str("Pass") }
            Some(cell) => { res.push_str(&cell.to_string()) }
        }
        write!(f, "{}", res)
    }
}
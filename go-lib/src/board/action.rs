use board::grid::{GoCell, Grid};
use display::goshow::GoShow;

#[derive(Debug, Copy, Hash, Clone, Eq, PartialEq)]
pub enum GoAction {
    Pass,
    Cell(usize, usize),
}


impl GoAction {
    pub fn x(&self) -> Option<usize> {
        match self {
            GoAction::Pass => { None }
            GoAction::Cell(x, _y) => Some(*x)
        }
    }

    pub fn y(&self) -> Option<usize> {
        match self {
            GoAction::Pass => None,
            GoAction::Cell(_x, y) => Some(*y)
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

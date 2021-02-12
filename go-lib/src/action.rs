use crate::cell::GoCell;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GoAction {
    pub cell: Option<GoCell>
}


impl GoAction {
    pub(crate) fn play_at(x: usize, y: usize) -> GoAction {
        GoAction { cell: Some(GoCell { x, y }) }
    }
    fn pass() -> GoAction {
        GoAction { cell: None }
    }
}

use crate::board::GoCell;

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

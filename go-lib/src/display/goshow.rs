use std::borrow::Borrow;
use std::ops::Deref;

use bit_set::BitSet;

use crate::board::go_state::GoState;
use board::stones::group::GoGroup;
use board::stones::grouprc::GoGroupRc;
use board::stones::stone::Stone;
use display::range::Range2;
use rust_tools::screen::layout::layout::LayoutRc;
use sgf::sgf_export::Sequence;
use go_rules::go_action::GoAction;

pub trait GoShow {
    fn sgf(board: &GoState) -> Sequence;
    fn history(board: &GoState) -> LayoutRc;
    fn board(board: &GoState) -> LayoutRc;
    fn board_range(board: &GoState, range: Range2) -> LayoutRc;
    fn group_layout(board: &GoState, group: &GoGroupRc) -> LayoutRc;
    fn group(board: &GoState, group: &GoGroup) -> String;

    fn grouprc(board: &GoState, group: &GoGroupRc) -> String {
        Self::group(board, group.borrow().deref())
    }

    fn cell(xy: (usize, usize)) -> String;
    fn cells(board: &GoState, stone: Stone, cells: &BitSet) -> String;
    fn stone(stone: Stone) -> String;
    fn action(action: &GoAction) -> String;
    fn column(x: usize) -> String;
    fn line(y: usize) -> String;
}


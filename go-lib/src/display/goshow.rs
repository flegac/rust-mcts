use bit_set::BitSet;

use board::action::GoAction;
use board::go_state::GoState;
use board::stones::groups::GoGroup;
use board::stones::stone::Stone;
use display::range::Range2;
use rust_tools::screen::layout::layout::LayoutRc;
use sgf::sgf_export::Sequence;

pub trait GoShow {
    fn game(board: &GoState)-> Sequence;

    fn board(board: &GoState) -> LayoutRc;
    fn board_range(board: &GoState, range: Range2) -> LayoutRc;
    fn group_layout(board: &GoState, group: &GoGroup) -> LayoutRc;
    fn group(board: &GoState, group: &GoGroup) -> String;
    fn cell(xy: (usize, usize)) -> String;
    fn cells(board: &GoState, stone: Stone, cells: &BitSet) -> String;
    fn stone(stone: Stone) -> String;
    fn action(action: &GoAction) -> String;
    fn column(x: usize) -> String;
    fn line(y: usize) -> String;
}


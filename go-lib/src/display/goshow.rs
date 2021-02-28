use bit_set::BitSet;

use action::GoAction;
use board::goboard::GoBoard;
use display::range::Range2;
use rust_tools::screen::layout::layout::LayoutRc;
use rust_tools::screen::screen::Screen;
use stones::group::GoGroup;
use stones::stone::Stone;

pub trait GoShow {
    fn board(board: &GoBoard) -> LayoutRc;
    fn board_range(board: &GoBoard, range: Range2) -> LayoutRc;
    fn group_layout(board: &GoBoard, group: &GoGroup) -> LayoutRc;
    fn group(board: &GoBoard, group: &GoGroup) -> String;
    fn cell(xy: (usize, usize)) -> String;
    fn cells(board: &GoBoard, stone: Stone, cells: &BitSet) -> String;
    fn stone(stone: Stone) -> String;
    fn action(action: &GoAction) -> String;
    fn column(x: usize) -> String;
    fn line(y: usize) -> String;
}


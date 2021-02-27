use bit_set::BitSet;

use action::GoAction;
use board::goboard::GoBoard;
use rust_tools::screen::screen::Screen;
use stones::group::GoGroup;
use stones::stone::Stone;

pub trait GoShow {
    fn board(board: &GoBoard) -> Screen;
    fn group(board: &GoBoard, group: &GoGroup) -> String;

    fn cell(xy: (usize, usize)) -> String;
    fn cells(board: &GoBoard, cells: &BitSet) -> String;
    fn stone(stone: Stone) -> String;
    fn action(action: &GoAction) -> String;
    fn column(x: usize) -> String;
    fn line(y: usize) -> String;
}


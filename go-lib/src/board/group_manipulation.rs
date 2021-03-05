use board::grid::GoCell;
use board::stones::grouprc::GoGroupRc;
use board::stones::stone::Stone;

pub trait GroupManipulation {
    fn place_stone(&mut self, cell: GoCell, stone: Stone) -> GoGroupRc;
    fn fusion_with(&mut self, cell: GoCell) -> (GoGroupRc, usize);
    fn split_with(&mut self, cell: GoCell) -> (GoGroupRc, Vec<GoGroupRc>);
    fn capture(&mut self, group: &GoGroupRc);
}

use board::grid::GoCell;
use board::stones::grouprc::GoGroupRc;

pub trait GroupManipulation {
    fn fusion_with(&mut self, cell: GoCell) -> (GoGroupRc, usize);
    fn split_with(&mut self, cell: GoCell) -> (GoGroupRc, Vec<GoGroupRc>);
    fn capture(&mut self, group: &GoGroupRc);
}

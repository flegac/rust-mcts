use indexmap::set::IndexSet;

use board::grid::{GoCell, Grid};
use board::stones::grouprc::GoGroupRc;
use board::stones::stone::Stone;
use bit_set::BitSet;

pub trait GroupAccess {
    fn fusion(&mut self, groups: &[GoGroupRc]) -> GoGroupRc;
    fn group_at(&self, cell: GoCell) -> &GoGroupRc;

    fn goban(&self) -> &Grid;

    fn capture(&mut self, group: &GoGroupRc);
    fn stone_at(&self, cell: GoCell) -> Stone;
    fn groups_by_stone_mut(&mut self, stone: Stone) -> &mut IndexSet<GoGroupRc>;
    fn groups_by_stone(&self, stone: Stone) -> &IndexSet<GoGroupRc>;
    fn update_liberties(&self, group: &GoGroupRc);
    fn adjacent_groups(&self, cell: GoCell) -> Vec<GoGroupRc>;

    fn fast_split_check(&self, old: &GoGroupRc, old_connections: &BitSet) -> bool;

}
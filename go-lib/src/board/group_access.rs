use bit_set::BitSet;
use indexmap::set::IndexSet;

use board::grid::{GoCell, Grid};
use board::stones::group::GoGroup;
use board::stones::grouprc::GoGroupRc;
use board::stones::stone::Stone;

pub trait GroupAccess {
    fn goban(&self) -> &Grid;
    fn stone_at(&self, cell: GoCell) -> Stone;

    fn group_at(&self, cell: GoCell) -> &GoGroupRc;
    //TODO: use Vec<GoGroupRc> in API (not IndexSet) ? watch for perfs
    fn groups_by_stone(&self, stone: Stone) -> &IndexSet<GoGroupRc>;
    fn groups_by_stone_mut(&mut self, stone: Stone) -> &mut IndexSet<GoGroupRc>;

    fn adjacent_groups(&self, cell: GoCell) -> Vec<GoGroupRc>;
    fn adjacent_allies_groups(&self, cell: GoCell, stone: Stone) -> Vec<GoGroupRc>;
    fn adjacent_ennemies_groups(&self, cell: GoCell, stone: Stone) -> Vec<GoGroupRc>;
    fn adjacent_empty_groups(&self, cell: GoCell) -> Vec<GoGroupRc>;

    fn update_liberties(&self, group: &GoGroupRc);
}

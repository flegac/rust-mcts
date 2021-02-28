use std::collections::hash_map::RandomState;
use std::collections::HashSet;

use board::grid::{GoCell, Grid};
use stones::grouprc::GoGroupRc;
use stones::stone::Stone;

pub trait GroupAccess {
    fn goban(&self) -> &Grid;

    fn capture(&mut self, group: &GoGroupRc);
    fn fusion(&mut self, groups: &[GoGroupRc]) -> GoGroupRc;

    fn group_at(&self, cell: GoCell) -> &GoGroupRc;
    fn stone_at(&self, cell: GoCell) -> Stone;
    fn groups_by_stone_mut(&mut self, stone: Stone) -> &mut HashSet<GoGroupRc>;
    fn groups_by_stone(&self, stone: Stone) -> &HashSet<GoGroupRc>;
    fn update_liberties(&self, group: &GoGroupRc);
    fn adjacent_groups(&self, cell: GoCell) -> Vec<GoGroupRc>;
}
use std::collections::hash_map::RandomState;
use std::collections::HashSet;

use board::goboard::GroupAccess;
use board::grid::{GoCell, Grid};
use stones::group::GoGroup;
use stones::grouprc::GoGroupRc;
use stones::stone::Stone;

pub struct BoardGroups {
    groups: Vec<GoGroupRc>,
    blacks: HashSet<GoGroupRc>,
    whites: HashSet<GoGroupRc>,
    nones: HashSet<GoGroupRc>,
}

impl BoardGroups {
    pub fn new() -> BoardGroups {
        BoardGroups {
            groups: vec![],
            blacks: HashSet::new(),
            whites: HashSet::new(),
            nones: HashSet::new(),
        }
    }


    pub(crate) fn update_group(&mut self, group: &GoGroupRc) {
        assert!(!group.borrow().cells.is_empty());
        for c in group.borrow().cells.iter() {
            self.groups[c] = group.clone();
        }
        self.update_group_color(&group);
    }


    pub(crate) fn reset_board_group(&mut self, group:&GoGroupRc) {
        self.groups.clear();
        self.whites.clear();
        self.blacks.clear();
        self.nones.clear();
        self.groups.resize_with(group.borrow().stones(), || group.clone());
        self.update_group_color(group);
    }

    pub fn clear_group_color(&mut self, group: &GoGroupRc) {
        self.blacks.remove(group);
        self.whites.remove(group);
        self.nones.remove(group);
    }

    pub(crate) fn update_group_color(&mut self, group: &GoGroupRc) {
        assert!(!group.borrow().is_empty());

        self.blacks.remove(group);
        self.whites.remove(group);
        self.nones.remove(group);
        self.groups_by_stone_mut(group.borrow().stone).insert(group.clone());
    }
}

impl GroupAccess for BoardGroups {
    fn group_at(&self, cell: GoCell) -> &GoGroupRc {
        &self.groups[cell]
    }

    fn stone_at(&self, cell: GoCell) -> Stone {
        self.group_at(cell).borrow().stone
    }

    fn groups_by_stone_mut(&mut self, stone: Stone) -> &mut HashSet<GoGroupRc, RandomState> {
        match stone {
            Stone::None => &mut self.nones,
            Stone::Black => &mut self.blacks,
            Stone::White => &mut self.whites
        }
    }


    fn groups_by_stone(&self, stone: Stone) -> &HashSet<GoGroupRc, RandomState> {
        match stone {
            Stone::None => &self.nones,
            Stone::Black => &self.blacks,
            Stone::White => &self.whites
        }

        // self.groups.iter()
        //     .filter(|&g| g.borrow().stone == stone)
        //     .unique()
        //     .map(|g| g.clone())
        //     .collect_vec()
    }
}

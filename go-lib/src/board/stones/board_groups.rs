use std::borrow::{Borrow, BorrowMut};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use bit_set::BitSet;
use indexmap::set::IndexSet;
use itertools::Itertools;

use board::go::Go;
use board::grid::{GoCell, Grid};
use board::group_access::GroupAccess;
use board::stones::grouprc::GoGroupRc;
use board::stones::groups::GoGroup;
use board::stones::stone::Stone;
use display::range::Range2;
use graph_lib::topology::Topology;

#[derive(Clone)]
pub struct BoardGroups {
    id_gen: usize,
    goban: Grid,
    groups: Vec<GoGroupRc>,
    blacks: IndexSet<GoGroupRc>,
    whites: IndexSet<GoGroupRc>,
    nones: IndexSet<GoGroupRc>,
    pub(crate) empty_cells: BitSet,

}

// impl Clone for BoardGroups {
//     fn clone(&self) -> Self {
//         let mut res = BoardGroups {
//             id_gen: self.id_gen,
//             goban: self.goban.clone(),
//             groups: vec![],
//             blacks: IndexSet::new(),
//             whites: IndexSet::new(),
//             nones: IndexSet::new(),
//             empty_cells: self.empty_cells.clone(),
//         };
//         for g in self.groups.iter() {
//             let gg = g.borrow();
//             let rc = GoGroupRc::from(gg.clone());
//             res.groups.push(rc.clone());
//             match gg.stone {
//                 Stone::None => {
//                     res.nones.insert(rc.clone());
//                 }
//                 Stone::Black => {
//                     res.blacks.insert(rc.clone());
//                 }
//                 Stone::White => {
//                     res.whites.insert(rc.clone());
//                 }
//             }
//
//         }
//
//         res
//     }
// }


impl BoardGroups {
    pub fn new(goban: Grid) -> BoardGroups {
        let ggg = GoGroup::from_goban(&goban);
        let empty_cells = goban.vertices().clone();
        let mut res = BoardGroups {
            id_gen: 0,
            goban,
            empty_cells,
            groups: vec![],
            blacks: IndexSet::new(),
            whites: IndexSet::new(),
            nones: IndexSet::new(),
        };
        let group = res.new_group(ggg);
        res.groups.resize_with(group.borrow().stones(), || group.clone());
        res.nones.insert(group.clone());
        res
    }

    pub fn group_range(&self, group: &GoGroupRc) -> Range2 {
        group.borrow().cells.iter()
            .map(|c| self.goban().xy(c))
            .fold(Range2::empty(), |c, v| c.merge(v))
    }

    pub fn reset(&mut self) {
        self.id_gen = 0;
        self.empty_cells = self.goban.vertices().clone();
        self.groups.clear();
        self.blacks.clear();
        self.whites.clear();
        self.nones.clear();
        let group = self.new_group(GoGroup::from_goban(&self.goban));
        self.groups.resize_with(group.borrow().stones(), || group.clone());
        self.nones.insert(group.clone());
    }


    pub fn new_group(&mut self, mut group: GoGroup) -> GoGroupRc {
        group.id = self.id_gen;
        self.id_gen += 1;
        GoGroupRc::from(group)
    }

    pub(crate) fn update_group(&mut self, group: &GoGroupRc) {
        assert!(!group.borrow().cells.is_empty());
        for c in group.borrow().cells.iter() {
            self.groups[c] = group.clone();
        }
        self.update_group_color(&group);
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
    fn goban(&self) -> &Grid {
        &self.goban
    }

    fn capture(&mut self, group: &GoGroupRc) {
        assert!(!group.borrow().is_empty());
        group.borrow_mut().set_stone(Stone::None);

        self.blacks.remove(group);
        self.whites.remove(group);
        self.nones.insert(group.clone());

        self.empty_cells.union_with(&group.borrow().cells);
    }

    fn fusion(&mut self, groups: &[GoGroupRc]) -> GoGroupRc {
        assert!(!groups.is_empty());
        //forget all stones
        for g in groups {
            self.clear_group_color(g);
        }

        //create one unique group
        let group = groups
            .iter()
            .map(GoGroupRc::clone)
            .fold1(|g1, g2| {
                g1.borrow_mut().add_group(g2.borrow().deref());
                g1
            })
            .unwrap();

        // add the final group
        self.update_group(&group);
        group
    }

    fn group_at(&self, cell: GoCell) -> &GoGroupRc {
        &self.groups[cell]
    }

    fn stone_at(&self, cell: GoCell) -> Stone {
        self.group_at(cell).borrow().stone
    }


    fn groups_by_stone_mut(&mut self, stone: Stone) -> &mut IndexSet<GoGroupRc, RandomState> {
        match stone {
            Stone::None => &mut self.nones,
            Stone::Black => &mut self.blacks,
            Stone::White => &mut self.whites
        }
    }

    fn groups_by_stone(&self, stone: Stone) -> &IndexSet<GoGroupRc, RandomState> {
        match stone {
            Stone::None => &self.nones,
            Stone::Black => &self.blacks,
            Stone::White => &self.whites
        }
    }

    fn update_liberties(&self, group: &GoGroupRc) {
        let go = Go::new(self);
        let mut adjacents = go.adjacent_cells(&group.borrow().cells);
        adjacents.intersect_with(&self.empty_cells);
        group.borrow_mut().liberties = adjacents.len();
    }

    fn adjacent_groups(&self, cell: usize) -> Vec<GoGroupRc> {
        self.goban.edges(cell)
            .iter()
            .map(|c| self.group_at(c))
            .unique()
            .map(|g| g.clone())
            .collect_vec()
    }
}

use std::borrow::{Borrow, BorrowMut};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};

use bit_set::BitSet;
use indexmap::set::IndexSet;
use itertools::Itertools;
use log::LevelFilter;

use board::grid::{GoCell, Grid};
use board::group_access::GroupAccess;
use board::stones::group::GoGroup;
use board::stones::grouprc::GoGroupRc;
use board::stones::stone::Stone;
use display::display::GoDisplay;
use display::goshow::GoShow;
use display::range::Range2;
use go_rules::go::Go;
use graph_lib::algo::flood::Flood;
use graph_lib::graph::GFlood;
use graph_lib::topology::Topology;
use rust_tools::screen::layout::layout::L;

use crate::board::group_manipulation::GroupManipulation;

#[derive(Debug, Clone)]
pub struct BoardGroups {
    id_gen: usize,
    goban: Grid,
    groups: Vec<GoGroupRc>,
    blacks: IndexSet<GoGroupRc>,
    whites: IndexSet<GoGroupRc>,
    nones: IndexSet<GoGroupRc>,
    pub(crate) empty_cells: BitSet,
}

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

    pub(crate) fn add_group(&mut self, group: &GoGroupRc) {
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


    fn fast_split_check(&self, old: &GoGroupRc, old_connections: &BitSet) -> bool {
        let to_visit = old.borrow().cells.clone();
        let topology = |c: GoCell| to_visit.contains(c);
        let old_cell = to_visit.iter().next().unwrap();
        let check_connection = |visited: &BitSet| old_connections.is_subset(visited);
        let visited = GFlood::new().flood_check(
            self.goban(), old_cell, &topology, &check_connection,
        );
        !check_connection(&visited)
    }

    fn split_group_with(&mut self, cell: GoCell, group: &GoGroupRc) -> Vec<GoGroup> {
        let mut res = vec![];

        //remove spliting cell
        let cells = BitSet::from_iter([cell].iter().map(|&x| x as usize));
        res.push(group.borrow_mut().split_remove(cells));

        while !group.borrow().is_empty() {
            let test = |x| group.borrow().cells.contains(x);
            let from = group.borrow().cells.iter().next().unwrap();
            let extracted_cells = GFlood::new().flood(self.goban(), from, &test);
            res.push(group.borrow_mut().split_remove(extracted_cells));
        }
        assert_eq!(group.borrow().stones(), 0);
        res
    }
}

impl GroupManipulation for BoardGroups {

    fn place_stone(&mut self, cell: GoCell, stone: Stone) -> GoGroupRc {
        assert_eq!(self.stone_at(cell), Stone::None);
        let new_stone = self.new_group(GoGroup::from_cells(stone, &[cell]));
        let rc = self.group_at(cell).clone();
        let groups = self.groups_by_stone_mut(Stone::None).borrow_mut();
        groups.remove(&rc);

        self.add_group(&new_stone);
        self.empty_cells.remove(cell);
        new_stone
    }

    fn fusion_with(&mut self, cell: GoCell) -> (GoGroupRc, usize) {
        let old_cell_group = self.group_at(cell);
        assert_eq!(old_cell_group.borrow().stones(), 1);

        let stone = old_cell_group.borrow().stone;
        let mut groups = self.adjacent_allies_groups(cell, stone);
        groups.push(old_cell_group.clone());

        //create one unique group
        let group = groups
            .iter()
            .map(GoGroupRc::clone)
            .fold1(|g1, g2| {
                g1.borrow_mut().add_group(g2.borrow().deref());
                g1
            })
            .unwrap();

        //forget all stones
        for g in groups.iter() {
            self.clear_group_color(g);
        }
        // add the final group
        self.add_group(&group);
        (group, groups.len())
    }

    fn split_with(&mut self, cell: GoCell) -> (GoGroupRc, Vec<GoGroupRc>) {
        let old = self.group_at(cell).clone();
        let res = self
            .split_group_with(cell, &old)
            .into_iter()
            .map(|g| self.new_group(g))
            .collect_vec();

        self.clear_group_color(&old);
        for g in res.iter() {
            //TODO: useless remove !
            old.borrow_mut().remove_group(g.borrow().deref());
            self.add_group(g);
        }

        (old, res)
    }

    fn capture(&mut self, group: &GoGroupRc) {
        assert!(!group.borrow().is_empty());
        group.borrow_mut().set_stone(Stone::None);

        self.blacks.remove(group);
        self.whites.remove(group);
        self.nones.insert(group.clone());

        self.empty_cells.union_with(&group.borrow().cells);
    }
}

impl GroupAccess for BoardGroups {
    fn group_at(&self, cell: GoCell) -> &GoGroupRc {
        &self.groups[cell]
    }


    fn goban(&self) -> &Grid {
        &self.goban
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

    fn adjacent_allies_groups(&self, cell: GoCell, stone: Stone) -> Vec<GoGroupRc> {
        self.adjacent_groups(cell).into_iter()
            .filter(|g| g.borrow().stone == stone)
            .collect_vec()
    }


    fn adjacent_enemies_groups(&self, cell: GoCell, stone: Stone) -> Vec<GoGroupRc> {
        self.adjacent_groups(cell).into_iter()
            .filter(|g| g.borrow().stone == stone.switch())
            .collect_vec()
    }

    fn adjacent_empty_groups(&self, cell: GoCell) -> Vec<GoGroupRc> {
        self.adjacent_groups(cell).into_iter()
            .filter(|g| g.borrow().stone == Stone::None)
            .collect_vec()
    }
}

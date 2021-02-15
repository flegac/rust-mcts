use core::fmt;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::RandomState;
use std::iter::FromIterator;
use std::ops::Deref;

use bit_set::BitSet;
use itertools::Itertools;

use board::goban::{Goban, GoCell};
use stones::group::GoGroup;
use stones::grouprc::GoGroupRc;
use stones::stone::Stone;

use crate::action::GoAction;

pub(crate) struct GoBoard<> {
    pub(crate) goban: Goban,
    groups: HashMap<GoCell, GoGroupRc>,
}


impl GoBoard {
    pub(crate) fn new(goban: Goban) -> Self {
        let mut board = GoBoard {
            goban,
            groups: HashMap::new(),
        };
        board.update_board_with_group(&mut GoGroupRc::new(Stone::None, board.goban.cells.clone()));
        board
    }


    fn stone_at(&self, cell: &GoCell) -> Stone {
        self.group_at(cell).borrow().stone
    }

    fn flood(&self, stone: Stone, cell: GoCell) -> BitSet {
        let mut visited = BitSet::new();
        let mut to_visit = BitSet::new();
        to_visit.insert(cell);

        while !to_visit.is_empty() {
            let mut connected = BitSet::new();
            for c in to_visit.iter() {
                self.goban.adjacents(c).iter()
                    .filter(|a| self.stone_at(a) == stone)
                    .filter(|&a| !visited.contains(a))
                    .for_each(|c| {
                        connected.insert(c);
                    });
            }


            visited.union_with(&connected);
            to_visit = connected;
        }
        visited
    }

    fn split(&self, group: &GoGroupRc) {
        let to_visit = &group.borrow().cells;
        let groups: Vec<GoGroup> = vec![];
        //TODO: split a group by connectivity
        while !to_visit.is_empty() {}
    }

    pub fn play_at(&mut self, cell: GoCell, stone: Stone) {
        let mut new_group = GoGroupRc::from_cell(stone, cell);
        let old = self.group_at(&cell);
        old.borrow_mut().remove_group(&new_group.borrow());
        //TODO: check old group connectivity & split if needed

        // update new group
        self.goban.adjacents(cell).iter()
            .map(|c| self.group_at(&c))
            .filter(|&g| g.borrow().stone == stone)
            .for_each(|g| {
                new_group.borrow_mut().add_group(g.borrow().deref())
            });


        //updating board with new group
        self.update_board_with_group(&new_group);

        // kill groups
        self.goban.adjacents(cell).iter()
            .map(|c| self.group_at(&c))
            .filter(|&g| g.borrow().stone == stone.switch())
            .for_each(|g| self.death_test(g));
    }

    pub(crate) fn group_at(&self, cell: &GoCell) -> &GoGroupRc {
        self.groups.get(&cell).unwrap()
    }

    pub(crate) fn count_stones(&self, stone: Stone) -> usize {
        self.groups.values()
            .filter(|&g| g.borrow().stone == stone)
            .unique()
            .map(|g| g.borrow().size())
            .sum()
    }

    fn death_test(&self, group: &GoGroupRc) {
        self.update_group_liberties(group);

        if self.is_dead(group) {
            println!("KILLED! {}", group);
            group.borrow_mut().set_stone(Stone::None)
        }
    }

    fn is_dead(&self, group: &GoGroupRc) -> bool {
        group.borrow().liberties.is_empty()
    }


    fn update_group_liberties(&self, group: &GoGroupRc) {
        let mut adjacents = BitSet::new();
        for c in group.borrow().cells.iter() {
            adjacents.union_with(&self.goban.adjacents(c));
        }
        adjacents.difference_with(&group.borrow().cells);

        group.borrow_mut().liberties.clear();
        for x in adjacents.iter()
            .filter(|c| self.group_at(c).borrow().stone == Stone::None) {
            group.borrow_mut().liberties.insert(x);
        }
    }


    fn update_board_with_group(&mut self, group: &GoGroupRc) {
        for c in group.borrow().cells.iter() {
            self.groups.insert(c, group.clone());
        }
    }
}


impl fmt::Display for GoBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::new();

        let size = self.goban.size;
        for x in 0..size {
            for y in 0..size {
                let g = self.group_at(&self.goban.cell(x, y));
                res.push_str(&g.borrow().stone.to_string());
                res.push_str(" ");
            }
            res.push_str("\n");
        }
        write!(f, "{}", res)
    }
}

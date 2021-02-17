use core::fmt;
use std::collections::{HashMap, HashSet};
use std::iter::{Filter, FromIterator, Map};
use std::ops::Deref;

use bit_set::{BitSet, Iter};
use itertools::Itertools;

use board::goban::{Goban, GoCell};
use board::stats::GoBoardStats;
use stones::group::GoGroup;
use stones::grouprc::GoGroupRc;
use stones::stone::Stone;

pub(crate) struct GoBoard<> {
    pub(crate) goban: Goban,
    groups: HashMap<GoCell, GoGroupRc>,
    pub(crate) stats: GoBoardStats,
}

impl GoBoard {
    pub fn new(goban: Goban) -> Self {
        let mut board = GoBoard {
            goban,
            groups: HashMap::new(),
            stats: GoBoardStats::new(),
        };
        board.update_board_with_group(&mut GoGroupRc::new(Stone::None, board.goban.cells.clone()));
        board
    }


    pub fn play_at(&mut self, cell: GoCell, stone: Stone) {
        let new_group = GoGroupRc::from_cell(stone, cell);
        let old = self.group_at(&cell).clone();
        old.borrow_mut().remove_group(&new_group.borrow());
        for part in self.split(old) {
            self.update_board_with_group(&part);
        }


        // update new group
        self.goban.adjacents(cell).iter()
            .filter(|c| self.stone_at(c) == stone)
            .map(|c| self.group_at(&c))
            .unique()
            .for_each(|g| {
                new_group.borrow_mut().add_group(g.borrow().deref())
            });


        //updating board with new group
        self.update_board_with_group(&new_group);

        // kill groups
        let deads = self.goban.adjacents(cell)
            .iter()
            .map(|c| self.group_at(&c).clone())
            .filter(|g| g.borrow().stone == stone.switch())
            .filter(|g| self.is_dead(g))
            .collect_vec();
        for g in deads.iter().unique()
        {
            if g.borrow().stone != Stone::None {
                self.capture_group(&g);
            }
        }

        self.update_stats();
    }


    pub fn group_at(&self, cell: &GoCell) -> &GoGroupRc {
        self.groups.get(&cell).unwrap()
    }


    pub fn stone_at(&self, cell: &GoCell) -> Stone {
        self.group_at(cell).borrow().stone
    }

    pub fn split(&self, g: GoGroupRc) -> Vec<GoGroupRc> {
        let mut res = vec![];

        while !g.borrow().is_empty() {
            let g1 = self.next_split(&g);
            g.borrow_mut().remove_group(&g1.borrow());
            res.push(g1);
        }


        res
    }

    fn get_territory_owner(&self, group: &GoGroupRc) -> Stone {
        let border = group.borrow().liberties.iter()
            .map(|c| self.stone_at(&c))
            .unique()
            .collect_vec();
        if border.len() >= 2 || border.is_empty() {
            Stone::None
        } else {
            border.get(0).unwrap().clone()
        }
    }

    fn count_territory(&self, stone: Stone) -> usize {

        //TODO: fix that
        self.groups.values()
            .filter(|&g| g.borrow().stone == Stone::None)
            .unique()
            .filter(|&g| self.get_territory_owner(g) == stone)
            .map(|g| g.borrow().size())
            .sum()
    }

    fn count_stones(&self, stone: Stone) -> usize {
        self.groups.values()
            .filter(|&g| g.borrow().stone == stone)
            .unique()
            .map(|g| g.borrow().size())
            .sum()
    }

    fn count_groups(&self, stone: Stone) -> usize {
        self.groups.values()
            .filter(|&g| g.borrow().stone == stone)
            .unique()
            .count()
    }

    fn capture_group(&mut self, group: &GoGroupRc) {
        println!("CAPTURED: {}", group);

        match group.borrow().stone {
            Stone::None => {
                panic!("capturing empty cells !");
            }
            Stone::Black => self.stats.black.captured += group.borrow().size(),
            Stone::White => self.stats.white.captured += group.borrow().size(),
        }
        group.borrow_mut().set_stone(Stone::None)
    }

    fn is_dead(&self, group: &GoGroupRc) -> bool {
        self.update_group_liberties(group);
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

    fn next_split(&self, group: &GoGroupRc) -> GoGroupRc {
        let to_visit = &group.borrow().cells;
        let test = |c: GoCell| to_visit.contains(c);

        let cell = to_visit.iter().next().unwrap();
        let cells = self.goban.flood(cell, &test);
        GoGroupRc::new(group.borrow().stone, cells)
    }

    fn update_stats(&mut self) {
        self.stats.black.stones = self.count_stones(Stone::Black);
        self.stats.black.groups = self.count_groups(Stone::Black);
        self.stats.white.stones = self.count_stones(Stone::White);
        self.stats.white.groups = self.count_groups(Stone::White);
        self.stats.none.stones = self.count_stones(Stone::None);
        self.stats.none.groups = self.count_groups(Stone::None);
    }


    fn write_score(&self, res: &mut String) {
        res.push_str("black: territories=");
        res.push_str(&self.count_territory(Stone::Black).to_string());
        res.push_str(", captured:=");
        res.push_str(&self.stats.black.captured.to_string());
        res.push_str("\n");

        res.push_str("white: territories=");
        res.push_str(&self.stats.white.captured.to_string());
        res.push_str(", captured:=");
        res.push_str(&self.count_territory(Stone::White).to_string());
        res.push_str("\n");
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


        self.write_score(&mut res);

        res.push_str(&self.stats.to_string());
        res.push_str("\n");

        write!(f, "{}", res)
    }
}

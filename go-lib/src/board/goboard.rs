use core::fmt;
use proc_macro::Group;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::{HashSet, LinkedList};
use std::collections::hash_map::RandomState;
use std::ops::{Deref, DerefMut};

use bit_set::BitSet;
use fixed_typed_arena::Arena;
use itertools::Itertools;

use action::GoAction;
use board::grid::{GoCell, Grid};
use board::stats_board::BoardStats;
use go_display::GoDisplay;
use graph_lib::algo::flood::Flood;
use graph_lib::graph::GFlood;
use graph_lib::topology::Topology;
use screen::Screen;
use stones::group::GoGroup;
use stones::grouprc::GoGroupRc;
use stones::stone::Stone;

pub struct GoBoard {
    arena: Arena<GoGroup>,
    pub stone: Stone,

    //groups
    groups: Vec<GoGroupRc>,

    blacks: HashSet<GoGroupRc>,
    whites: HashSet<GoGroupRc>,
    nones: HashSet<GoGroupRc>,

    pub(crate) empty_cells: GoGroup,

    //graph
    pub(crate) goban: Grid,

    pub(crate) stats: BoardStats,
    pub(crate) flood: RefCell<GFlood>,
}

impl GoBoard {
    pub fn new(goban: Grid) -> Self {
        let empty_cells = GoGroup::from_goban(&goban);
        let mut board = GoBoard {
            arena: Arena::new(),
            goban,
            groups: vec![],

            blacks: HashSet::new(),
            whites: HashSet::new(),
            nones: HashSet::new(),

            stats: BoardStats::new(),
            stone: Stone::Black,
            empty_cells,
            flood: RefCell::new(GFlood::new()),
        };
        board.reset();
        board
    }

    pub fn reset(&mut self) {
        self.stats = BoardStats::new();
        self.stone = Stone::Black;
        self.empty_cells = GoGroup::from_goban(&self.goban);
        let board_group = self.new_group(GoGroup::from_goban(&self.goban));
        self.groups.clear();
        self.whites.clear();
        self.blacks.clear();
        self.nones.clear();
        self.update_group(board_group);
    }

    pub fn group_at(&self, cell: GoCell) -> &GoGroupRc {
        &self.groups[cell]
    }

    pub fn stone_at(&self, cell: GoCell) -> Stone {
        self.group_at(cell).borrow().stone
    }

    pub fn groups_by_stone_mut(&mut self, stone: Stone) -> &mut HashSet<GoGroupRc, RandomState> {
        match stone {
            Stone::None => &mut self.nones,
            Stone::Black => &mut self.blacks,
            Stone::White => &mut self.whites
        }
    }


    pub fn groups_by_stone(&self, stone: Stone) -> &HashSet<GoGroupRc, RandomState> {
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

    pub fn end_game(&self) -> bool {
        let limit = self.vertex_number();
        self.stats.round > limit || self.stats.none.groups == 0
    }


    pub fn play(&mut self, action: GoAction) {
        log::trace!("NEW PLAY: {} @ {}", self.stone, action);

        match action {
            GoAction::Pass => {}
            GoAction::Cell(x, y) => {
                let cell = self.goban.cell(x, y);
                self.place_stone(cell, self.stone);
            }
        }

    }

    pub fn place_stone(&mut self, cell: GoCell, stone: Stone) {
        let before = self.screen(true);

        assert!(self.stone_at(cell) == Stone::None);

        self.handle_old_empty_group(cell);

        //fusion allied groups
        let new_group = self.fusion_allied_groups(cell, stone);

        self.kill_ennemy_groups(cell, stone);

        //FIXME: do not allow this case to happen !
        self.check_autokill(new_group);

        self.stats.round += 1;

        let after = self.screen(true);
        let mut full = Screen::new(before.width * 2 + 1, before.height);
        full.draw(0, 0, &before);
        full.draw(before.width as i32 + 1 , 0, &after);
        log::trace!("\n{}", full.to_string());

        self.check_correctness();
    }

    pub fn check_correctness(&self) {
        assert_eq!(self.stats.round, self.stats.compute_round());
        assert_eq!(self.empty_cells.stones(), self.stats.none.stones);
        assert_eq!(
            self.stats.black.stones
                + self.stats.white.stones
                + self.stats.none.stones,
            self.vertex_number()
        );
        self.stats.assert_eq(&BoardStats::from_board(self));
    }

    pub fn update_score<F>(&mut self, scorer: F)
        where F: Fn(Stone, &GoBoard) -> usize
    {
        self.stats.black.territory = scorer(Stone::Black, self);
        self.stats.white.territory = scorer(Stone::White, self);
    }

    pub fn score(&self, stone: Stone) -> usize {
        let stats = self.stats.for_stone(stone);
        let territory = stats.territory;
        let captures = stats.captured;
        territory + captures
    }

    fn handle_old_empty_group(&mut self, cell: usize) {
        self.empty_cells.cells.remove(cell);

        let old = self.group_at(cell).clone();
        let mut old_connections = self.goban.edges(cell).clone();
        old_connections.intersect_with(&old.borrow().cells);
        old_connections.remove(cell); // TODO: useless ?

        log::trace!("handle_old_empty_group: {}", GoDisplay::cells(self, &old_connections));

        match old_connections.len() {
            0 => {
                // old group was only the last placed cell
                // self.stats.rem_group(old.borrow().deref());
                self.stats.none.groups -= 1;
                self.clear_group_color(&old);
                old.borrow_mut().cells.remove(cell);
            }
            1 => {
                // old group connexity is preserved !
                old.borrow_mut().cells.remove(cell);
            }
            _ => {
                old.borrow_mut().cells.remove(cell);

                // maybe we have cut the old group
                let need_split = self.fast_split_check(&old, &old_connections);

                if need_split {
                    old.borrow_mut().cells.insert(cell);
                    self.clear_group_color(&old);
                    old.borrow_mut().cells.remove(cell);

                    self.stats.rem_group(old.borrow().deref());

                    let parts = old.borrow_mut().split(&self);
                    for part in parts {
                        log::trace!("- new empty group: {}", GoDisplay::group(self, &part));
                        self.update_group(self.new_group(part));
                    }
                }
            }
        }
    }

    fn fast_split_check(&self, old: &GoGroupRc, old_connections: &BitSet) -> bool {
        let to_visit = old.borrow().cells.clone();
        let topology = |c: GoCell| to_visit.contains(c);
        let old_cell = to_visit.iter().next().unwrap();
        let check_connection = |visited: &BitSet| {
            // false
            old_connections.is_subset(visited)
        };
        let visited = self.flood.borrow_mut().flood_check(
            &self.goban, old_cell, &topology, &check_connection,
        );
        let split_needed = !check_connection(&visited);
        split_needed
    }


    fn fusion_allied_groups(&mut self, cell: usize, stone: Stone) -> GoGroupRc {
        let new_group = self.new_group(GoGroup::from_cell(stone, cell));
        self.goban.edges(cell).iter()
            .filter(|&c| self.stone_at(c) == stone)
            .map(|c| self.group_at(c))
            // .unique()
            .map(|g| g.clone())
            .sorted()
            .dedup()
            .for_each(|g: GoGroupRc| {
                new_group.borrow_mut().add_group(g.borrow().deref());
                self.clear_group_color(&g);
                self.stats.rem_group(&g.borrow());
            });
        self.update_group(new_group.clone());
        new_group
    }

    fn check_autokill(&mut self, new_group: GoGroupRc) {
        log::trace!("check_autokill...");
        if self.try_capture(new_group.clone()) {
            log::trace!("AUTOKILL MOVE! {}", new_group);
        }
    }

    fn kill_ennemy_groups(&mut self, cell: usize, stone: Stone) {
        log::trace!("kill_ennemy_groups");
        self.goban.edges(cell).iter()
            .filter(|&c| self.stone_at(c) == stone.switch())
            .map(|c| self.group_at(c))
            .map(|g| g.clone())
            .sorted()
            .dedup()
            .for_each(|g: GoGroupRc| {
                self.try_capture(g.clone());
            });
    }

    fn try_capture(&mut self, group: GoGroupRc) -> bool {
        //TODO: this is sufficient ?
        // g.borrow_mut().liberties -= 1;
        // let libs = g.borrow_mut().liberties - 1;
        group.borrow_mut().update_liberties(self);
        // assert_eq!(libs, g.borrow().liberties);

        if group.borrow().is_dead() {
            log::trace!("captured {} stones: {:?}",
                        group.borrow().stones(),
                        group.borrow().cells);
            match group.borrow().stone {
                Stone::None => {}
                Stone::Black => self.stats.black.captured += group.borrow().stones(),
                Stone::White => self.stats.white.captured += group.borrow().stones(),
            }

            //update ancient color group
            self.clear_group_color(&group);
            self.stats.rem_group(group.borrow().deref());

            // remove stone from group & update None groups
            group.borrow_mut().set_stone(Stone::None);
            self.empty_cells.add_group(group.borrow().deref());

            self.update_group_color(&group);
            self.stats.none.groups += 1;

            true
        } else {
            false
        }
    }

    fn clear_group_color(&mut self, group: &GoGroupRc) {
        self.blacks.remove(group);
        self.whites.remove(group);
        self.nones.remove(group);
    }


    fn update_group(&mut self, group: GoGroupRc) {
        assert!(!group.borrow().cells.is_empty());

        let cell_number = self.vertex_number();
        if group.borrow().stones() == cell_number {
            self.groups.resize_with(cell_number, || group.clone());
        } else {
            for c in group.borrow().cells.iter() {
                self.groups[c] = group.clone();
            }
        }
        self.update_group_color(&group);
        self.stats.add_group(group.clone().borrow().deref());
    }

    fn update_group_color(&mut self, group: &GoGroupRc) {
        assert!(!group.borrow().is_empty());

        self.blacks.remove(group);
        self.whites.remove(group);
        self.nones.remove(group);
        self.groups_by_stone_mut(group.borrow().stone).insert(group.clone());
    }

    fn new_group(&self, group: GoGroup) -> GoGroupRc {
        // self.arena.alloc(group)
        GoGroupRc::from(group)
    }
}


impl Topology for GoBoard {
    fn vertices(&self) -> &BitSet<u32> {
        self.goban.vertices()
    }
    fn edges(&self, v: usize) -> &BitSet<u32> {
        self.goban.edges(v)
    }
}


#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use std::sync::Arc;

    use bit_set::BitSet;
    use rpool::{Pool, Poolable, PoolScaleMode};

    use board::goboard::GoBoard;
    use board::grid::Grid;
    use graph_lib::algo::flood::Flood;
    use graph_lib::graph::GFlood;
    use graph_lib::topology::Topology;
    use stones::group::GoGroup;
    use stones::grouprc::GoGroupRc;
    use stones::stone::Stone;

    #[test]
    fn stone_groups() {
        let goban = Grid::new(7);
        let board = GoBoard::new(goban);

        let mut cells = BitSet::new();
        for cell in &[
            board.goban.cell(0, 0),
            board.goban.cell(0, 3),
            board.goban.cell(3, 0)
        ] {
            cells.insert(*cell);
        }

        let group = board.new_group(GoGroup::from_cell(
            Stone::Black,
            cells.iter().next().unwrap(),
        ));

        assert_eq!(group.borrow().stones(), 3);
    }

    #[test]
    fn board_cell_id() {
        let goban = Grid::new(7);


        goban.apply(|c| {
            let (x, y) = goban.xy(c);
            let c2 = goban.cell(x, y);
            let (x2, y2) = goban.xy(c2);

            assert_eq!(c, c2);
            assert_eq!(x, x2);
            assert_eq!(y, y2);
        });
    }
}

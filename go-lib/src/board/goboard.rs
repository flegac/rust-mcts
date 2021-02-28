use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::ops::Deref;

use bit_set::BitSet;
use graph_lib::algo::flood::Flood;
use graph_lib::graph::GFlood;
use graph_lib::topology::Topology;
use itertools::Itertools;
use log::LevelFilter;

use action::GoAction;
use board::grid::{GoCell, Grid};
use board::stats_board::BoardStats;
use display::display::GoDisplay;
use display::goshow::GoShow;
use display::range::Range2;
use rust_tools::screen::layout::layout::L;
use rust_tools::screen::layout::template::Template;
use rust_tools::screen::screen::Screen;
use stones::group::GoGroup;
use stones::grouprc::GoGroupRc;
use stones::stone::Stone;

pub struct GoBoard {
    // template: Template,

    //game state
    pub stone: Stone,
    pass_sequence: usize,
    ko: Option<GoCell>,

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
            // template: Template::new(L::str("")),

            stone: Stone::Black,
            pass_sequence: 0,
            ko: None,

            goban,
            groups: vec![],

            blacks: HashSet::new(),
            whites: HashSet::new(),
            nones: HashSet::new(),

            stats: BoardStats::new(),
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
        let double_pass = self.pass_sequence >= 2;
        self.stats.round > limit || self.stats.none.groups == 0  // || double_pass
    }

    pub fn play(&mut self, action: GoAction) {
        log::trace!("NEW PLAY: {} @ {}", self.stone, action);

        let before = if log::max_level() <= LevelFilter::Trace {
            GoDisplay::board(self)
        } else {
            L::str("")
        };


        match action {
            GoAction::Pass => {
                self.pass_sequence += 1;
            }
            GoAction::Cell(x, y) => {
                self.pass_sequence = 0;
                let cell = self.goban.cell(x, y);
                self.place_stone(cell, self.stone);
            }
        }
        self.stone = self.stone.switch();
        self.stats.round += 1;

        if log::max_level() <= LevelFilter::Trace {
            log::trace!("\n{}", L::hori(vec![
                before,
                L::str(" - padding - "),
                GoDisplay::board(self)
            ]).to_string());
        }

        self.check_correctness();
    }

    pub fn group_range(&self, group: &GoGroupRc) -> Range2 {
        group.borrow().cells.iter()
            .map(|c| self.goban.xy(c))
            .fold(Range2::empty(), |c, v| c.merge(v))
    }

    pub fn place_stone(&mut self, cell: GoCell, stone: Stone) {
        let splited_groups = self.handle_old_empty_group(cell);

        let new_group = self.fusion_allied_groups(cell, stone);
        self.kill_ennemy_groups(cell, stone);

        self.check_autokill(new_group);


        if log::max_level() <= LevelFilter::Trace {
            if !splited_groups.is_empty() {
                log::trace!("SPLITS:\n{}", L::hori(splited_groups.iter()
                    .map(|g| self.group_range(g))
                    .map(|range| GoDisplay::board_range(self, range))
                    .collect_vec()).to_string());
            }
        }
    }

    fn check_correctness(&self) {
        assert_eq!(self.empty_cells.stones(), self.stats.none.stones);
        assert_eq!(
            self.stats.black.stones
                + self.stats.white.stones
                + self.stats.none.stones,
            self.vertex_number()
        );
        //FIXME: remove this (costly) check !
        // self.stats.assert_eq(&BoardStats::from_board(self));
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

    fn handle_old_empty_group(&mut self, cell: usize) -> Vec<GoGroupRc> {
        self.empty_cells.cells.remove(cell);

        let old = self.group_at(cell).clone();
        let mut old_connections = self.goban.edges(cell).clone();
        old_connections.intersect_with(&old.borrow().cells);
        old_connections.remove(cell); // TODO: useless ?

        if log::max_level() <= LevelFilter::Trace {
            log::trace!("handle_old_empty_group: {}",
                        GoDisplay::cells(self,
                                         Stone::None,
                                         &old_connections));
        }

        match old_connections.len() {
            0 => {
                // old group was only the last placed cell
                // self.stats.rem_group(old.borrow().deref());
                self.stats.none.groups -= 1;
                self.clear_group_color(&old);
                old.borrow_mut().cells.remove(cell);
                vec![]
            }
            1 => {
                // old group connexity is preserved !
                old.borrow_mut().cells.remove(cell);
                vec![]
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
                    if log::max_level() <= LevelFilter::Trace {
                        log::trace!("- old (remove): {}\n{}", old, self.stats_str());
                    }
                    let new_groups = old.borrow_mut()
                        .split(&self)
                        .into_iter()
                        .map(|g| self.new_group(g))
                        .collect_vec();

                    for g in new_groups.iter() {
                        if log::max_level() <= LevelFilter::Trace {
                            log::trace!("-{}", GoDisplay::group(self, &g.borrow()));
                        }
                        self.update_group(g.clone());
                    }
                    new_groups
                } else {
                    vec![]
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
        let new_group = self.new_group(GoGroup::from_cells(stone, &[cell]));
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
                if log::max_level() <= LevelFilter::Trace {
                    log::trace!("- fusion (remove): {}\n{}", g.borrow(), self.stats_str());
                }
            });
        self.update_group(new_group.clone());
        new_group
    }

    fn check_autokill(&mut self, new_group: GoGroupRc) {
        //FIXME: do not allow this case to happen !
        log::trace!("check_autokill...");
        if self.try_capture(new_group.clone()) {
            if log::max_level() <= LevelFilter::Trace {
                log::trace!("AUTOKILL MOVE! {}", new_group);
            }
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
            if log::max_level() <= LevelFilter::Trace {
                log::trace!("captured : {}\n {}",
                            GoDisplay::group(&self, group.borrow().deref()),
                            GoDisplay::group_layout(&self, group.borrow().deref()).to_string(),
                );
            }
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
        if log::max_level() <= LevelFilter::Trace {
            log::trace!("add: {}\n{}", group, self.stats_str());
        }
    }

    fn update_group_color(&mut self, group: &GoGroupRc) {
        assert!(!group.borrow().is_empty());

        self.blacks.remove(group);
        self.whites.remove(group);
        self.nones.remove(group);
        self.groups_by_stone_mut(group.borrow().stone).insert(group.clone());
    }

    fn new_group(&self, group: GoGroup) -> GoGroupRc {
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
    use graph_lib::algo::flood::Flood;
    use graph_lib::graph::GFlood;
    use graph_lib::topology::Topology;

    use board::goboard::GoBoard;
    use board::grid::Grid;
    use rpool::{Pool, Poolable, PoolScaleMode};
    use stones::group::GoGroup;
    use stones::grouprc::GoGroupRc;
    use stones::stone::Stone;

    #[test]
    fn stone_groups() {
        let goban = Grid::new(7);
        let board = GoBoard::new(goban);

        let mut cells = [
            board.goban.cell(0, 0),
            board.goban.cell(0, 3),
            board.goban.cell(3, 0)
        ];

        let group = board.new_group(GoGroup::from_cells(Stone::Black, &cells));

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

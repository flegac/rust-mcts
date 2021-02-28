use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::ops::Deref;

use bit_set::BitSet;
use graph_lib::algo::flood::Flood;
use graph_lib::graph::GFlood;
use graph_lib::topology::Topology;
use itertools::Itertools;
use log::LevelFilter;

use action::GoAction;
use board::go::Go;
use board::grid::{GoCell, Grid};
use display::display::GoDisplay;
use display::goshow::GoShow;
use display::range::Range2;
use mcts_lib::state::{GameResult, State};
use rust_tools::screen::layout::layout::{L, LayoutRc};
use rust_tools::screen::layout::template::Template;
use rust_tools::screen::screen::Screen;
use stones::group::GoGroup;
use stones::grouprc::GoGroupRc;
use stones::stone::Stone;
use groups::board_groups::BoardGroups;
use groups::group_access::GroupAccess;
use stats::board_stats::{BoardStats, FullStats};
use stats::stone_score::StoneScore;
use stats::stone_stats::StoneStats;

pub struct GoState {
    // template: Template,

    //sgf state
    pub stone: Stone,
    pass_sequence: usize,
    ko: Option<GoCell>,
    pub(crate) stats: BoardStats,
    pub history: Vec<GoAction>,

    //groups
    pub(crate) gg: BoardGroups,

    //graph
    pub(crate) flood: RefCell<GFlood>,

}

impl GoState {
    pub fn new(size: usize) -> Self {
        let goban = Grid::new(size);
        let mut board = GoState {
            stone: Stone::Black,
            pass_sequence: 0,
            ko: None,
            stats: BoardStats::new(),
            history: vec![],

            gg: BoardGroups::new(goban),
            flood: RefCell::new(GFlood::new()),
        };
        board.stats.add_group(&board.gg.empty_cells);
        board
    }

    pub fn end_game(&self) -> bool {
        let limit = self.vertex_number();
        let double_pass = self.pass_sequence >= 2;
        self.stats.round > limit || self.stats(Stone::None).groups == 0  // || double_pass
    }


    pub fn play(&mut self, action: GoAction) {
        let backup = self.play_start(action);

        match action {
            GoAction::Pass => {
                self.pass_sequence += 1;
            }
            GoAction::Cell(x, y) => {
                self.pass_sequence = 0;
                let cell = self.goban().cell(x, y);
                self.place_stone(cell, self.stone);
            }
        }
        self.stone = self.stone.switch();
        self.stats.round += 1;
        self.history.push(action);

        self.play_end(backup);
    }

    pub fn place_stone(&mut self, cell: GoCell, stone: Stone) {
        let old_groups = self.try_split_empty_cells(cell);
        let new_group = self.fusion_allied_groups(cell, stone);
        //new_group.liberties = allies.liberties.sum() + 4 - 2 * allies_connections
        //ennemy.liberties -= 1
        self.kill_ennemy_groups(cell, stone);
        // /!\ allies touching dead groups must be updated !

        log::trace!("AUTO KILL : checking...");
        self.try_capture(new_group.clone());

        if log::max_level() <= LevelFilter::Trace {
            if !old_groups.is_empty() {
                log::trace!("SPLITS:\n{}", L::hori(old_groups.iter()
                    .map(|g| self.gg.group_range(g))
                    .map(|range| GoDisplay::board_range(self, range))
                    .collect_vec()).to_string());
            }
        }
    }

    pub fn update_score<F>(&mut self, scorer: F)
        where F: Fn(Stone, &GoState) -> usize {
        self.set_territory(Stone::Black, scorer(Stone::Black, self));
        self.set_territory(Stone::White, scorer(Stone::White, self));
    }


    fn try_split_empty_cells(&mut self, cell: usize) -> Vec<GoGroupRc> {
        self.gg.empty_cells.cells.remove(cell);

        let old = self.group_at(cell).clone();
        let mut old_connections = self.goban().edges(cell).clone();
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
                self.stats(Stone::None).groups -= 1;
                self.gg.clear_group_color(&old);
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
                    self.gg.clear_group_color(&old);
                    old.borrow_mut().cells.remove(cell);

                    self.stats.rem_group(old.borrow().deref());
                    if log::max_level() <= LevelFilter::Trace {
                        log::trace!("- old (remove): {}\n{}", old, self.stats);
                    }
                    let new_groups = old.borrow_mut()
                        .split(&self)
                        .into_iter()
                        .map(|g| self.gg.new_group(g))
                        .collect_vec();

                    for g in new_groups.iter() {
                        if log::max_level() <= LevelFilter::Trace {
                            log::trace!("-{}", GoDisplay::group(self, &g.borrow()));
                        }
                        self.update_group(&g);
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
        let check_connection = |visited: &BitSet| old_connections.is_subset(visited);
        let visited = self.flood.borrow_mut().flood_check(
            self.goban(), old_cell, &topology, &check_connection,
        );
        !check_connection(&visited)
    }


    fn fusion_allied_groups(&mut self, cell: usize, stone: Stone) -> GoGroupRc {
        let mut groups = self.adjacent_groups(cell).into_iter()
            .filter(|g| g.borrow().stone == stone)
            .collect_vec();
        let new_group = self.gg.new_group(GoGroup::from_cells(stone, &[cell]));
        self.update_group(&new_group);
        groups.push(new_group);
        self.fusion(&groups)
    }

    fn kill_ennemy_groups(&mut self, cell: usize, stone: Stone) {
        self.adjacent_groups(cell).into_iter()
            .filter(|g| g.borrow().stone == stone.switch())
            .for_each(|g| {
                self.try_capture(g);
            });
    }


    fn try_capture(&mut self, group: GoGroupRc) {
        self.update_liberties(&group);
        if group.borrow().is_dead() {
            self.capture(&group);
        }
    }

    fn update_group(&mut self, group: &GoGroupRc) {
        assert!(!group.borrow().cells.is_empty());
        self.gg.update_group(&group);
        self.stats.add_group(group.clone().borrow().deref());
        if log::max_level() <= LevelFilter::Trace {
            log::trace!("add: {}\n{}", group, self.stats);
        }
    }

    fn play_start(&mut self, action: GoAction) -> LayoutRc {
        log::trace!("NEW PLAY: {} @ {}", self.stone, action);
        if log::max_level() <= LevelFilter::Trace {
            GoDisplay::board(self)
        } else {
            L::str("")
        }
    }

    fn play_end(&mut self, backup: LayoutRc) {
        if log::max_level() <= LevelFilter::Trace {
            log::trace!("\n{}", L::hori(vec![
                backup,
                L::str(" - padding - "),
                GoDisplay::board(self)
            ]).to_string());
        }
        self.check_correctness();
    }

    fn check_correctness(&self) {
        assert_eq!(self.gg.empty_cells.stones(), self.stats(Stone::None).stones);
        assert_eq!(
            self.stats(Stone::Black).stones
                + self.stats(Stone::White).stones
                + self.stats(Stone::None).stones,
            self.vertex_number()
        );
        //FIXME: remove this (costly) check !
        // self.stats.assert_eq(&BoardStats::from_board(self));
    }
}


impl State<GoAction> for GoState {
    fn reset(&mut self) {
        self.stone = Stone::Black;
        self.pass_sequence = 0;
        self.ko = None;
        self.history.clear();
        self.gg.reset();
        self.stats = BoardStats::new();
        self.stats.add_group(&self.gg.empty_cells);
    }

    fn result(&self) -> Option<GameResult> {
        if self.end_game() {
            let player = self.score(self.stone).score();
            let opponent = self.score(self.stone.switch()).score();
            let res = match player.cmp(&opponent) {
                Ordering::Less => GameResult::Lose,
                Ordering::Equal => GameResult::Draw,
                Ordering::Greater => GameResult::Win
            };
            Some(res)
        } else {
            None
        }
    }

    fn actions(&self) -> Vec<GoAction> {
        let mut actions = self.gg.empty_cells.cells.iter()
            .map(|c| self.goban().xy(c))
            .map(|(x, y)| GoAction::Cell(x, y))
            .collect_vec();
        actions.push(GoAction::Pass);
        actions
    }

    fn apply(&mut self, action: GoAction) {
        self.play(action);
    }
}

impl Topology for GoState {
    fn vertices(&self) -> &BitSet<u32> {
        self.goban().vertices()
    }
    fn edges(&self, v: usize) -> &BitSet<u32> {
        self.goban().edges(v)
    }
}

impl FullStats for GoState {
    fn score(&self, stone: Stone) -> StoneScore {
        self.stats.score(stone)
    }

    fn stats(&self, stone: Stone) -> StoneStats {
        self.stats.stats(stone)
    }

    fn add_prisoners(&mut self, stone: Stone, n: usize) {
        self.stats.add_prisoners(stone, n)
    }

    fn set_territory(&mut self, stone: Stone, n: usize) {
        self.stats.set_territory(stone, n)
    }
}

impl GroupAccess for GoState {
    fn goban(&self) -> &Grid {
        &self.gg.goban()
    }

    fn capture(&mut self, group: &GoGroupRc) {
        self.stats.add_prisoners(group.borrow().stone, group.borrow().stones());
        self.stats.rem_group(group.borrow().deref());
        self.stats.for_stone_mut(Stone::None).groups += 1;

        self.gg.capture(group);
        if log::max_level() <= LevelFilter::Trace {
            log::trace!("DEAD GROUP : {}\n {}",
                        GoDisplay::group(&self, group.borrow().deref()),
                        GoDisplay::group_layout(&self, group.borrow().deref()).to_string(),
            );
        }
    }

    fn fusion(&mut self, groups: &[GoGroupRc]) -> GoGroupRc {
        let group = self.gg.fusion(groups);
        self.stats.for_stone_mut(group.borrow().stone).groups -= (groups.len() - 1);
        if log::max_level() <= LevelFilter::Trace {
            log::trace!("FUSION {}:\n{}", group.borrow(), self.stats);
        }
        group
    }

    fn group_at(&self, cell: usize) -> &GoGroupRc {
        self.gg.group_at(cell)
    }


    fn stone_at(&self, cell: usize) -> Stone {
        self.gg.stone_at(cell)
    }

    fn groups_by_stone_mut(&mut self, stone: Stone) -> &mut HashSet<GoGroupRc, RandomState> {
        self.gg.groups_by_stone_mut(stone)
    }

    fn groups_by_stone(&self, stone: Stone) -> &HashSet<GoGroupRc, RandomState> {
        self.gg.groups_by_stone(stone)
    }

    fn update_liberties(&self, group: &GoGroupRc) {
        self.gg.update_liberties(group)
    }

    fn adjacent_groups(&self, cell: usize) -> Vec<GoGroupRc> {
        self.gg.adjacent_groups(cell)
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

    use board::go_state::GoState;
    use board::grid::Grid;
    use stones::group::GoGroup;
    use stones::grouprc::GoGroupRc;
    use stones::stone::Stone;


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
use std::cmp::Ordering;
use std::collections::hash_map::RandomState;
use std::ops::Deref;

use bit_set::BitSet;
use indexmap::set::IndexSet;
use itertools::Itertools;
use log::LevelFilter;

use board::grid::{GoCell, Grid};
use board::group_access::GroupAccess;
use board::stats::board_stats::{BoardStats, FullStats};
use board::stats::stone_score::StoneScore;
use board::stats::stone_stats::StoneStats;
use board::stones::board_groups::BoardGroups;
use board::stones::group::GoGroup;
use board::stones::grouprc::GoGroupRc;
use board::stones::stone::Stone;
use display::display::GoDisplay;
use display::goshow::GoShow;
use graph_lib::algo::flood::Flood;
use graph_lib::graph::GFlood;
use graph_lib::topology::Topology;
use mcts_lib::rules::{GameResult, Rules};
use rust_tools::screen::layout::layout::{L, Layout, LayoutRc};

use crate::board::go_state::GoState;
use go_rules::go_action::GoAction;
use go_rules::go::Go;

impl Rules<GoAction> for GoState {
    fn fork(&self) -> Self {
        let copy = self.clone();
        copy.check_correctness();

        let show = L::hori(vec![
            GoDisplay::board(self),
            GoDisplay::board(&copy),
        ]);

        log::trace!("FORKING STATE:\n{}", show.to_string());
        copy
    }

    fn reset(&mut self) {
        self.stone = Stone::Black;
        self.pass_sequence = 0;
        self.ko = None;
        self.history.clear();
        self.gg.reset();
        self.stats = BoardStats::new(self.goban());
    }

    fn result(&self) -> Option<GameResult> {
        let limit = self.vertex_number();
        let double_pass = self.pass_sequence >= 2;
        let end_game = self.stats.round > limit
            || self.stats(Stone::None).groups == 0
            || double_pass;

        if end_game {
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
        let mut actions = self.gg.empty_cells
            .iter()
            .map(|c| self.goban().xy(c))
            .map(|(x, y)| GoAction::Cell(x, y))
            .collect_vec();
        actions.push(GoAction::Pass);
        actions
    }

    fn apply_action(&mut self, action: GoAction) {
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
}

pub trait GoRules {
    fn try_split_empty_cells(&mut self, cell: usize) -> Vec<GoGroupRc>;
    fn fusion_allied_groups(&mut self, cell: usize, stone: Stone) -> GoGroupRc;
    fn kill_ennemy_groups(&mut self, cell: usize, stone: Stone);
    fn try_capture(&mut self, group: GoGroupRc);
    fn update_group(&mut self, group: &GoGroupRc);
    fn place_stone(&mut self, cell: GoCell, stone: Stone);
    fn update_score(&mut self);
    fn play_start(&mut self, action: GoAction) -> LayoutRc;
    fn play_end(&mut self, backup: LayoutRc);
    fn check_correctness(&self);
}

impl GoRules for GoState {
    fn try_split_empty_cells(&mut self, cell: usize) -> Vec<GoGroupRc> {
        self.gg.empty_cells.remove(cell);

        let old = self.group_at(cell).clone();
        let mut old_connections = self.goban().edges(cell).clone();
        old_connections.intersect_with(&old.borrow().cells);
        old_connections.remove(cell); // TODO: useless ?

        if log::max_level() <= LevelFilter::Trace {
            log::trace!("SPLIT EMPTY GROUP: {}",
                        GoDisplay::cells(self,
                                         Stone::None,
                                         &old_connections));
        }

        let res = match old_connections.len() {
            0 => {
                self.stats(Stone::None).groups -= 1;
                self.gg.clear_group_color(&old);
                old.borrow_mut().cells.remove(cell);
                vec![]
            }
            1 => {
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
                            log::trace!("-{}", GoDisplay::grouprc(self, g));
                        }
                        self.update_group(&g);
                    }
                    new_groups
                } else {
                    vec![]
                }
            }
        };
        res
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
        log::trace!("KILLING GROUPS:");
        self.adjacent_groups(cell).into_iter()
            .filter(|g| g.borrow().stone == stone.switch())
            .for_each(|g| {
                self.try_capture(g);
            });
    }


    fn try_capture(&mut self, group: GoGroupRc) {
        self.update_liberties(&group);
        if group.borrow().is_dead() {
            log::trace!("{}", GoDisplay::grouprc(self, &group));
            self.capture(&group);
            log::trace!("{}", GoDisplay::grouprc(self, &group));
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

    fn place_stone(&mut self, cell: GoCell, stone: Stone) {
        let old_groups = self.try_split_empty_cells(cell);
        let new_group = self.fusion_allied_groups(cell, stone);
        //new_group.liberties = allies.liberties.sum() + 4 - 2 * allies_connections
        //ennemy.liberties -= 1
        self.kill_ennemy_groups(cell, stone);
        // /!\ allies touching dead stones must be updated !

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

    fn update_score(&mut self) {
        let go = Go::new(&self.gg);
        self.stats.set_territory(Stone::Black, go.count_territory(Stone::Black));
        self.stats.set_territory(Stone::White, go.count_territory(Stone::White));
    }


    fn play_start(&mut self, action: GoAction) -> LayoutRc {
        log::trace!("NEW PLAY: {} @ {}\n{}",
                    self.stone, action,
                    GoDisplay::board(self).to_string());

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
        for &s in [Stone::Black, Stone::White, Stone::None].iter() {
            let n1 = Go::new(&self.gg).count_stones(s);
            let n2 = self.stats.stats(s).stones;
            assert_eq!(n1, n2)
        }

        assert_eq!(self.gg.empty_cells.len(), self.stats(Stone::None).stones);
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

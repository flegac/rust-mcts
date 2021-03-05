use std::borrow::BorrowMut;
use std::cmp::Ordering;
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
use go_rules::go::Go;
use go_rules::go_action::GoAction;
use graph_lib::algo::flood::Flood;
use graph_lib::graph::GFlood;
use graph_lib::topology::Topology;
use mcts_lib::rules::{GameResult, Rules};
use rust_tools::screen::layout::layout::{L, LayoutRc};

use crate::board::go_state::GoState;
use crate::board::group_manipulation::GroupManipulation;

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
        self.stats = BoardStats::new(self.gg.goban());
    }

    fn result(&self) -> Option<GameResult> {
        let limit = self.gg.goban().vertex_number();
        let double_pass = self.pass_sequence >= 2;
        let end_game = self.stats.round > limit
            || self.stats(Stone::None).groups == 0
            || double_pass;

        if end_game {
            let player = self.stats.score(self.stone).score();
            let opponent = self.stats.score(self.stone.switch()).score();
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
            .map(|c| self.gg.goban().xy(c))
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
                let cell = self.gg.goban().cell(x, y);
                self.place_stone_and_update(cell, self.stone);
            }
        }
        self.stone = self.stone.switch();
        self.stats.round += 1;
        self.history.push(action);

        self.play_end(backup);
    }
}

pub trait GoRules {
    fn place_stone_and_update(&mut self, cell: GoCell, stone: Stone);

    fn place_stone(&mut self, cell: GoCell, stone: Stone) -> GoGroupRc;
    fn kill_ennemy_groups(&mut self, cell: usize, stone: Stone);

    fn group_self_connected_cells(&self, cell: GoCell) -> BitSet;

    fn try_capture(&mut self, group: GoGroupRc);
    fn update_group(&mut self, group: &GoGroupRc);
    fn play_start(&mut self, action: GoAction) -> LayoutRc;
    fn play_end(&mut self, backup: LayoutRc);

    fn update_score(&mut self);
    fn check_correctness(&self);
}

impl GoRules for GoState {
    fn place_stone_and_update(&mut self, cell: GoCell, stone: Stone) {
        log::trace!("PLACE STONE:\n{}", self.stats);

        //split old empty group
        let (old, new_empty_groups) = self.gg.split_with(cell);
        self.stats.for_stone_mut(Stone::None).groups += new_empty_groups.len();
        self.stats.for_stone_mut(Stone::None).groups -= 1;
        log::trace!("AFTER SPLIT_GROUP:\n{}", self.stats);


        if log::max_level() <= LevelFilter::Trace {
            if !new_empty_groups.is_empty() {
                log::trace!("SPLITS:\n{}", L::hori(new_empty_groups.iter()
                    .map(|g| self.gg.group_range(g))
                    .map(|range| GoDisplay::board_range(self, range))
                    .collect_vec()).to_string());
            }
        }


        // place new stone
        let new_stone = self.place_stone(cell, stone);
        // because we split old group
        self.stats.for_stone_mut(Stone::None).groups -= 1;
        self.update_group(&new_stone);



        //fusion allies groups
        let (fusion_group, old_groups) = self.gg.fusion_with(cell);
        self.stats.for_stone_mut(stone).groups += 1;
        self.stats.for_stone_mut(stone).groups -= old_groups;
        if log::max_level() <= LevelFilter::Trace {
            log::trace!("AFTER FUSION {}:\n{}", GoDisplay::grouprc(self, &fusion_group), self.stats);
        }

        self.kill_ennemy_groups(cell, stone);
        log::trace!("AFTER KILL_ENNEMY_GROUPS:\n{}", self.stats);

        self.try_capture(fusion_group.clone());
        log::trace!("AFTER AUTO_KILL:\n{}", self.stats);
    }

    fn place_stone(&mut self, cell: GoCell, stone: Stone) -> GoGroupRc {
        assert_eq!(self.gg.stone_at(cell), Stone::None);
        let new_stone = self.gg.new_group(GoGroup::from_cells(stone, &[cell]));


        let rc = self.gg.group_at(cell).clone();
        let groups = self.gg.groups_by_stone_mut(Stone::None).borrow_mut();
        groups.remove(&rc);

        self.gg.update_group(&new_stone);
        self.gg.empty_cells.remove(cell);
        new_stone
    }

    fn kill_ennemy_groups(&mut self, cell: usize, stone: Stone) {
        log::trace!("KILLING GROUPS:");

        for g in self.gg.adjacent_ennemies_groups(cell, stone) {
            self.try_capture(g);
        }
    }

    fn group_self_connected_cells(&self, cell: GoCell) -> BitSet {
        let mut cells = self.gg.goban().edges(cell).clone();
        cells.intersect_with(&self.gg.group_at(cell).borrow().cells);
        cells
    }

    fn try_capture(&mut self, group: GoGroupRc) {
        self.gg.update_liberties(&group);
        if group.borrow().is_dead() {
            log::trace!("{}", GoDisplay::grouprc(self, &group));
            if log::max_level() <= LevelFilter::Trace {
                log::trace!("DEAD GROUP : {}\n{}\n{}",
                            GoDisplay::grouprc(&self, &group),
                            self.stats.to_string(),
                            GoDisplay::group_layout(&self, &group).to_string(),
                );
            }
            self.stats.add_prisoners(group.borrow().stone, group.borrow().stones());
            self.stats.rem_group(group.borrow().deref());
            self.stats.for_stone_mut(Stone::None).groups += 1;
            self.gg.capture(&group);
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

    fn update_score(&mut self) {
        let go = Go::new(&self.gg);
        self.stats.set_territory(Stone::Black, go.count_territory(Stone::Black));
        self.stats.set_territory(Stone::White, go.count_territory(Stone::White));
    }


    fn check_correctness(&self) {
        for &s in [Stone::Black, Stone::White, Stone::None].iter() {
            let n1 = Go::new(&self.gg).count_stones(s);
            let n2 = self.stats.stats(s).stones;
            assert_eq!(n1, n2, "{:?} stones", s)
        }

        assert_eq!(self.gg.empty_cells.len(), self.stats(Stone::None).stones);
        assert_eq!(
            self.stats(Stone::Black).stones
                + self.stats(Stone::White).stones
                + self.stats(Stone::None).stones,
            self.gg.goban().vertex_number()
        );
        //FIXME: remove this (costly) check !
        // self.stats.assert_eq(&BoardStats::from_board(self));
    }
}

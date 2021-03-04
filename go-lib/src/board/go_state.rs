use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
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
use go_rules::go_action::GoAction;
use graph_lib::algo::flood::Flood;
use graph_lib::graph::GFlood;
use graph_lib::topology::Topology;
use mcts_lib::rules::{GameResult, Rules};
use rust_tools::screen::layout::layout::{L, Layout, LayoutRc};

#[derive(Debug, Clone)]
pub struct GoState {
    pub stone: Stone,
    pub(crate) pass_sequence: usize,
    pub(crate) ko: Option<GoCell>,
    pub(crate) stats: BoardStats,
    pub history: Vec<GoAction>,

    //stones
    pub(crate) gg: BoardGroups,
}

impl GoState {
    pub fn new(size: usize) -> Self {
        let goban = Grid::new(size);
        let stats = BoardStats::new(&goban);
        let mut board = GoState {
            stone: Stone::Black,
            pass_sequence: 0,
            ko: None,
            stats,
            history: vec![],

            gg: BoardGroups::new(goban),
        };

        board
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
        if log::max_level() <= LevelFilter::Trace {
            log::trace!("DEAD GROUP : {}\n{}\n{}",
                        GoDisplay::grouprc(&self, group),
                        self.stats.to_string(),
                        GoDisplay::group_layout(&self, group).to_string(),
            );
        }
        self.stats.add_prisoners(group.borrow().stone, group.borrow().stones());
        self.stats.rem_group(group.borrow().deref());
        self.stats.for_stone_mut(Stone::None).groups += 1;
        self.gg.capture(group);
    }

    fn fusion(&mut self, groups: &[GoGroupRc]) -> GoGroupRc {
        let group = self.gg.fusion(groups);
        self.stats.for_stone_mut(group.borrow().stone).groups -= groups.len() - 1;
        if log::max_level() <= LevelFilter::Trace {
            log::trace!("FUSION {}:\n{}", GoDisplay::grouprc(self, &group), self.stats);
        }
        group
    }

    fn group_at(&self, cell: usize) -> &GoGroupRc {
        self.gg.group_at(cell)
    }


    fn stone_at(&self, cell: usize) -> Stone {
        self.gg.stone_at(cell)
    }

    fn groups_by_stone_mut(&mut self, stone: Stone) -> &mut IndexSet<GoGroupRc, RandomState> {
        self.gg.groups_by_stone_mut(stone)
    }

    fn groups_by_stone(&self, stone: Stone) -> &IndexSet<GoGroupRc, RandomState> {
        self.gg.groups_by_stone(stone)
    }

    fn update_liberties(&self, group: &GoGroupRc) {
        self.gg.update_liberties(group)
    }

    fn adjacent_groups(&self, cell: usize) -> Vec<GoGroupRc> {
        self.gg.adjacent_groups(cell)
    }

    fn fast_split_check(&self, old: &GoGroupRc, old_connections: &BitSet<u32>) -> bool {
        self.gg.fast_split_check(old, old_connections)
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use std::sync::Arc;

    use bit_set::BitSet;
    use log::LevelFilter;

    use board::go_action::GoAction;
    use board::grid::Grid;
    use display::display::GoDisplay;
    use display::goshow::GoShow;
    use go_rules::go_action::GoAction;
    use graph_lib::algo::flood::Flood;
    use graph_lib::graph::GFlood;
    use graph_lib::topology::Topology;
    use mcts_lib::rules::Rules;
    use rust_tools::loggers::init_logs;
    use rust_tools::screen::layout::layout::L;

    use crate::board::go_state::GoState;

    #[test]
    fn go_state_clone() {
        init_logs(LevelFilter::Debug);

        let mut state = GoState::new(5);


        for i in 0..4 {
            for j in 3..5 {
                state.apply_action(GoAction::Cell(i, j));
            }
        }

        let copy = state.clone();


        L::vert(vec![
            L::str(&format!("{:?}", state)),
            L::str(&format!("{:?}", copy)),
        ]).show();

        let mut stats = vec![state, copy];
        for a in vec![
            GoAction::Cell(2, 1),
            GoAction::Cell(1, 2),
            GoAction::Cell(2, 2),
            GoAction::Cell(1, 1),
        ] {
            L::hori(vec![
                GoDisplay::board(&stats[0]),
                L::str(" - padding - "),
                GoDisplay::board(&stats[1]),
            ]).show();
            for go in stats.iter_mut() {
                go.apply_action(a)
            }
            assert_eq!(format!("{:?}", stats[0]), format!("{:?}", stats[1]))
        }

        L::hori(vec![
            GoDisplay::board(&stats[0]),
            L::str(" - padding - "),
            GoDisplay::board(&stats[1]),
        ]).show();
    }
}

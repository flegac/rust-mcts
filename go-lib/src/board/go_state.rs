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
use board::stats::full_stats::{BoardStats, FullStats};
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
use go_rules::go::Go;

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
    pub fn stats(&self, stone: Stone) -> StoneStats {
        self.stats.stats(stone)
    }


    pub(crate) fn play_start(&mut self, action: GoAction) -> LayoutRc {
        log::trace!("NEW PLAY: {} @ {}\n{}",
                    self.stone, action,
                    GoDisplay::board(self).to_string());

        if log::max_level() <= LevelFilter::Trace {
            GoDisplay::board(self)
        } else {
            L::str("")
        }
    }

    pub(crate) fn play_end(&mut self, backup: LayoutRc) {
        if log::max_level() <= LevelFilter::Trace {
            log::trace!("\n{}", L::hori(vec![
                backup,
                L::str(" - padding - "),
                GoDisplay::board(self)
            ]).to_string());
        }
        self.check_correctness();
    }

    pub(crate) fn check_correctness(&self) {
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

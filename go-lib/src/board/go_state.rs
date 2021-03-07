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
use go_rules::go::Go;
use go_rules::go_action::GoAction;
use graph_lib::algo::flood::Flood;
use graph_lib::graph::GFlood;
use graph_lib::topology::Topology;
use mcts_lib::rules::{GameResult, Rules};
use rust_tools::screen::layout::layout::{L, Layout, LayoutRc};

#[derive(Debug, Clone)]
pub struct GoState {
    pub stone: Stone,
    pub pass_sequence: usize,
    pub ko: Option<GoCell>,
    pub stats: BoardStats,
    pub history: Vec<GoAction>,

    //stones
    pub gg: BoardGroups,
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
        if log::max_level() >= LevelFilter::Trace {
            let layout = GoDisplay::board(self);
            log::trace!("NEW PLAY: {} @ {}\n{}", self.stone, action, layout.to_screen_str());
            layout
        } else {
            L::str("")
        }
    }

    pub(crate) fn play_end(&mut self, backup: LayoutRc) {
        if log::max_level() <= LevelFilter::Trace {
            log::trace!("\n{}", L::vert(vec![
                backup,
                GoDisplay::board(self)
            ]).to_screen_str());
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

extern crate chrono;
extern crate env_logger;
extern crate go_lib;
extern crate log;
extern crate mcts_lib;
extern crate rust_tools;

use std::{env, fs};
use std::borrow::Borrow;
use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::path::Path;

use constants::{BENCH, GOBAN_SIZE, LOG_LEVEL, SEED, SIM_FACTOR};
use go_lib::board::go_state::GoState;
use go_lib::board::group_access::GroupAccess;
use go_lib::board::stones::stone::Stone;
use go_lib::display::display::GoDisplay;
use go_lib::display::goshow::GoShow;
use go_lib::go_rules::go_action::GoAction;
use go_lib::go_rules::go_rules::GoRules;
use go_lib::mcts::capture_policy::CapturePolicy;
use go_lib::sgf::sgf_export::SGF;
use mcts_lib::explorator::Explorer;
use mcts_lib::mcts::Mcts;
use mcts_lib::policy::random_policy::RandomPolicy;
use mcts_lib::policy::win_score::WinScore;
use mcts_lib::rules::{Action, Rules};
use mcts_lib::sim_result::SimResult;
use rust_tools::bench::Bench;
use rust_tools::loggers::init_logs;
use rust_tools::screen::layout::layout::L;
use simulator::show_best_variant;

mod editor;
mod constants;
mod simulator;

pub fn main() {
    init_logs(LOG_LEVEL);
    simulator::reload_sgf();

    let selection_score = WinScore::new();
    let random_policy = RandomPolicy::new(SEED);
    // let capture_policy = CapturePolicy {
    //     other:&RandomPolicy::new(SEED)
    // };

    let mut explorer = Explorer::new(
        SIM_FACTOR,
        GoState::new(GOBAN_SIZE),
    );

    let mut stats = SimResult::new();
    let mut bench = Bench::with_speed(SIM_FACTOR as f32);
    let mut i = 0;
    // while bench.for_duration(BENCH.full_time) {
    while bench.for_iterations(2) {
        let res = explorer.explore(&random_policy, &selection_score);
        stats.merge(res.value.borrow().deref());
        i += 1;
        if i % 1000 == 0 {
            explorer.mcts_mut().selection(&selection_score);
            show_best_variant(&mut explorer);
        }
        if i == 100 {
            break;
        }
    }
    // explorer.mcts_mut().selection(&selection_score);
    show_best_variant(&mut explorer);
    log::info!("results: {}", stats);
    log::info!("{}", bench);

    simulator::save_sgf(explorer.mcts().state())
}
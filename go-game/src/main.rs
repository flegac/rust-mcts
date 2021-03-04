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
use go_lib::board::action::GoAction;
use go_lib::board::go_state::GoState;
use go_lib::board::group_access::GroupAccess;
use go_lib::board::stones::stone::Stone;
use go_lib::display::display::GoDisplay;
use go_lib::display::goshow::GoShow;
use go_lib::mcts::capture_policy::CapturePolicy;
use go_lib::sgf::sgf_export::SGF;
use mcts_lib::explorator::Explorator;
use mcts_lib::mcts::Mcts;
use mcts_lib::policy::random_policy::RandomPolicy;
use mcts_lib::policy::win_score::WinScore;
use mcts_lib::sim_result::SimResult;
use mcts_lib::state::{Action, State};
use rust_tools::bench::Bench;
use rust_tools::loggers::init_logs;
use rust_tools::screen::layout::layout::L;

mod editor;
mod constants;


fn load_sgf(filename: &Path) -> Result<String, String> {
    match fs::read_to_string(filename) {
        Ok(content) => {
            Ok(content)
        }
        Err(_) => Err(String::from("File not found !"))
    }
}

pub fn main() {
    init_logs(LOG_LEVEL);
    if let Ok(mut path) = env::current_dir() {
        path.push("output.sgf");
        println!("path: {:?}", path.as_path());
        if let Ok(game) = load_sgf(&path) {
            println!("game: {}", game);
        }
    }

    let selection_score = WinScore::new();
    let random_policy = RandomPolicy::new(SEED);
    // let capture_policy = CapturePolicy {
    //     other:&RandomPolicy::new(SEED)
    // };
    let mut explorator = Explorator::new(
        SIM_FACTOR,
        GoState::new(GOBAN_SIZE),
    );

    let mut stats = SimResult::new();
    let mut bench = Bench::with_speed(SIM_FACTOR as f32);
    let mut i = 0;
    while bench.for_duration(BENCH.full_time) {
        let res = explorator.explore(&random_policy, &selection_score);
        stats.merge(res.value.borrow().deref());
        i += 1;
        if i % 1000 == 0 {
            show_best_variant(&mut explorator);
        }
    }
    show_best_variant(&mut explorator);
    log::info!("results: {}", stats);
    log::info!("{}", bench);

    if let Ok(mut file) = File::create("full_game.sgf") {
        file.write_all(
            GoDisplay::sgf(explorator.mcts().state())
                .to_string().as_bytes()
        );
    }
}


pub fn show_best_variant(explorator: &mut Explorator<GoAction, GoState>) {
    let selection_score = WinScore::new();
    explorator.mcts_mut().selection(&selection_score);
    explorator.mcts_mut().state_mut().update_score();
    let board = explorator.mcts().state();
    L::hori(vec![
        GoDisplay::board(board),
        GoDisplay::history(board),
    ]).show();
    log::info!("root max depth: {}", explorator.mcts().borrow().root().max_depth());
}
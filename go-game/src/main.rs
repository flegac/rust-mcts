extern crate chrono;
extern crate env_logger;
extern crate go_lib;
extern crate log;
extern crate mcts_lib;
extern crate rust_tools;

use std::{env, fs};
use std::io::Write;
use std::path::Path;

use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;

use constants::{BENCH, GOBAN_SIZE, LOG_LEVEL, SEED, SIM_FACTOR};
use go_lib::board::go_state::GoState;
use go_lib::board::group_access::GroupAccess;
use go_lib::board::stones::stone::Stone;
use go_lib::sgf::sgf_export::SGF;
use mcts_lib::explorator::Explorator;
use mcts_lib::mcts::Mcts;
use mcts_lib::policy::random_policy::RandomPolicy;
use mcts_lib::policy::win_score::WinScore;
use rust_tools::bench::Bench;

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
    let sim_policy = RandomPolicy::new(SEED);
    let mut explorator = Explorator::new(
        SIM_FACTOR,
        GoState::new(GOBAN_SIZE),
    );

    let mut bench = Bench::with_speed(SIM_FACTOR as f32);
    let cursor = loop {
        let res = explorator.explore(&sim_policy, &selection_score);
        if bench.for_duration(BENCH.full_time) {
            break res;
        }
    };

    explorator.mcts_mut().state_mut().update_score();
    let mut mcts = explorator.mcts();
    log::info!("Board:\n{}", mcts.state());
    log::info!("results: {}", cursor);
    log::info!("{}", bench);

    let board = mcts.state();
    SGF::save(board.goban().size, Stone::Black, board.history.as_slice())
}


fn init_logs(level: LevelFilter) {
    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                     "{} [{}] - {}",
                     Local::now().format("%Y-%m-%dT%H:%M:%S"),
                     record.level(),
                     record.args()
            )
        })
        .filter(None, level)
        .init();
}

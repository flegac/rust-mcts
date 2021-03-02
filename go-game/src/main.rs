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
use mcts_lib::mcts::MState;
use mcts_lib::mymcts::MyMcts;
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

    let mut mcts = MyMcts::new(SIM_FACTOR);
    let mut root = mcts.get_state(GoState::new(GOBAN_SIZE));

    let mut bench = Bench::with_speed(SIM_FACTOR as f32);
    while bench.for_duration((BENCH.full_time)) {
        let mut round = Bench::with_speed(SIM_FACTOR as f32);
        while round.for_duration(BENCH.round_time) {
            mcts.explore(&mut root,
                         &sim_policy,
                         &selection_score);
        }
        root.state_mut().update_score();
        log::info!("Board:\n{}", root.state());
        log::info!("{} x {} | results: {}", SIM_FACTOR, round, mcts.root);
    }

    println!("{}", bench);
    let board = root.state();
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

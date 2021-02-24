extern crate chrono;
extern crate env_logger;
extern crate go_lib;
extern crate log;
extern crate mcts_lib;

use std::io::Write;

use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;

use bench::Bench;
use constants::{BENCH, GOBAN_SIZE, LOG_LEVEL};
use go_lib::board::go::Go;
use go_lib::gostate::GoState;
use mcts_lib::mymcts::MyMcts;
use mcts_lib::random_policy::RandomPolicy;

mod editor;
mod bench;
mod constants;


pub fn main() {
    init_logs(LOG_LEVEL);

    let policy = RandomPolicy::new(453);
    let state = GoState::new(GOBAN_SIZE);
    let mut mcts = MyMcts::new(state, policy);

    let mut bench = Bench::new(BENCH.full_time);
    while bench.looping() {
        let mut round = bench.spawn(BENCH.round_time);
        while round.looping_inc(None) {
            mcts.explore();
        }
        bench.inc_bench(&round);
        mcts.state.state.board.update_score(Go::count_territory);
        log::info!("Board:\n{}", mcts.state.state);
        log::info!("{} | results: {}", round, mcts.root);
    }

    mcts.explore();
    mcts.state.state.board.update_score(Go::count_territory);
    log::info!("Board:\n{}", mcts.state.state);
    log::info!("{} | results: {}", bench, mcts.root);
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

extern crate chrono;
extern crate env_logger;
extern crate go_lib;
extern crate log;
extern crate mcts_lib;

use std::fmt::{Display, Formatter};
use std::io::Write;

use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;

use bench::Bench;
use go_lib::constants::BENCH;
use go_lib::gostate::GoState;
use mcts_lib::mymcts::MyMcts;
use mcts_lib::random_policy::RandomPolicy;

mod editor;
mod bench;


pub fn main() {
    init_logs(LevelFilter::Debug);

    let policy = RandomPolicy::new(453);
    let mut state = GoState::new();
    let mut mcts = MyMcts::new(state, policy);

    let mut bench = Bench::new(BENCH.full_time);
    while bench.looping() {
        let mut round = Bench::new(BENCH.round_time);
        while round.looping() {
            mcts.explore();
            round.inc(1);
        }
        bench.inc_bench(&round);
        log::info!("{} | results: {}", round, mcts.root);
    }

    mcts.explore();
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

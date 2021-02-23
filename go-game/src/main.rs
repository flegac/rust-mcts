extern crate chrono;
extern crate env_logger;
extern crate go_lib;
extern crate log;
extern crate mcts_lib;

use std::fmt::{Display, Formatter};
use std::fmt;
use std::io::Write;

use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;

use bench::Bench;
use go_lib::action::GoAction;
use go_lib::constants::BENCH;
use go_lib::gostate::GoState;
use mcts_lib::mcts::Mcts;
use mcts_lib::mymcts::MyMcts;
use mcts_lib::random_policy::RandomPolicy;
use mcts_lib::state::State;

mod editor;
mod bench;


pub fn main() {
    init_logs();

    let policy = RandomPolicy::new(451);
    let mut mcts = MyMcts::new(policy);
    let mut state = GoState::new();

    let mut bench = Bench::new(BENCH.full_time);
    while bench.looping() {
        let mut round = Bench::new(BENCH.round_time);
        while round.looping() {
            state.reset();
            mcts.explore(&mut state);
            round.inc(1);
        }
        bench.inc_bench(&round);
        log::info!(
            "{} | results: {} wins, {} defeats, {} draws",
            round,
            mcts.root.value.borrow().wins,
            mcts.root.value.borrow().defeats(),
            mcts.root.value.borrow().draws,
        );
    }

    state.reset();
    mcts.explore(&mut state);
    log::info!("Board:\n{}", state);

    log::info!(
        "{} | results: {} wins, {} defeats, {} draws",
        bench,
        mcts.root.value.borrow().wins,
        mcts.root.value.borrow().defeats(),
        mcts.root.value.borrow().draws,
    );
}


fn init_logs() {
    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                     "{} [{}] - {}",
                     Local::now().format("%Y-%m-%dT%H:%M:%S"),
                     record.level(),
                     record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
}

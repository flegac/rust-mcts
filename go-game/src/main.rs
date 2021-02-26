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
use constants::{BENCH, GOBAN_SIZE, LOG_LEVEL, SEED, SIM_FACTOR};
use go_lib::board::go::Go;
use go_lib::gostate::GoState;
use mcts_lib::mcts::MState;
use mcts_lib::mymcts::MyMcts;
use mcts_lib::policy::win_score::WinScore;
use mcts_lib::policy::random_policy::RandomPolicy;

mod editor;
mod bench;
mod constants;



pub fn main() {
    init_logs(LOG_LEVEL);

    let selection_score = WinScore::new();
    let sim_policy = RandomPolicy::new(SEED);

    let mut mcts = MyMcts::new(SIM_FACTOR);
    let mut root = mcts.get_state(GoState::new(GOBAN_SIZE));

    let mut bench = Bench::new(BENCH.full_time);
    while bench.looping() {
        let mut round = bench.spawn(BENCH.round_time);
        while round.looping_inc(None) {
            mcts.explore(&mut root,
                         &sim_policy,
                         &selection_score);
        }
        bench.inc_bench(&round);
        root.state_mut().board.update_score(Go::count_territory);
        log::info!("Board:\n{}", root.state());
        log::info!("{} x {} | results: {}", SIM_FACTOR, round, mcts.root);
    }
    root.state_mut().board.update_score(Go::count_territory);
    log::info!("Board:\n{}", root.state());
    log::info!("results: {}", mcts.root);
    log::info!("{}\n{}", bench, bench.log_speed(SIM_FACTOR as f32));
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

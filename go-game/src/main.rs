extern crate chrono;
extern crate env_logger;
extern crate go_lib;
extern crate log;
extern crate mcts_lib;

use std::io::Write;
use std::time::Instant;

use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;

use go_lib::action::GoAction;
use go_lib::constants::BENCH;
use go_lib::gostate::GoState;
use mcts_lib::mcts::Mcts;
use mcts_lib::mymcts::MyMcts;
use mcts_lib::state::State;

mod editor;

pub fn main() {
    init_logs();
    let bench = Instant::now();

    let mut mcts: MyMcts<GoAction> = MyMcts::new(1234);

    let mut total_games = 0;

    while bench.elapsed() < BENCH.full_time {
        let round = Instant::now();
        let mut roud_games = 0;
        while round.elapsed() < BENCH.round_time {
            let mut state = GoState::initial();
            mcts.explore(&mut state);
            roud_games += 1;
        }
        log::info!(
            "Speed: {} games {:?} sec | results: {} wins, {} defeats, {} draws",
            roud_games,
            round.elapsed(),
            mcts.root.value.borrow().wins,
            mcts.root.value.borrow().defeats(),
            mcts.root.value.borrow().draws,
        );
        total_games += roud_games;
    }

    let duration = bench.elapsed();

    let mut state = GoState::initial();
    mcts.explore(&mut state);
    log::info!("Board:\n{}", state);

    log::info!(
        "Speed: {} games {:?} sec | results: {} wins, {} defeats, {} draws",
        total_games,
        duration,
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

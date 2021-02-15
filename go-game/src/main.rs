extern crate go_lib;
extern crate mcts_lib;

use std::time::Instant;

use go_lib::action::GoAction;
use go_lib::gostate::GoState;
use mcts_lib::mcts::Mcts;
use mcts_lib::mymcts::MyMcts;
use mcts_lib::state::State;

pub fn main() {
    let start = Instant::now();

    let mcts: MyMcts<GoAction, GoState> = MyMcts::new();

    let mut state = GoState::initial();

    mcts.explore(&mut state);

    println!("{}", state);

    let duration = start.elapsed();
    println!("duration: {:?}", duration);
}

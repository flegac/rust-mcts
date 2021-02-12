use go_lib::action::GoAction;
use go_lib::gostate::GoState;
use mcts_lib::mcts::Mcts;
use mcts_lib::mymcts::MyMcts;
use mcts_lib::state::State;

pub fn main() {
    let mcts: MyMcts<GoAction, GoState> = MyMcts::new();

    let mut state = GoState::initial();

    let res = mcts.explore(&mut state);

    println!("{:?}", res);

    println!("{}", state);
}

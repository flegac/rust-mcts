use crate::gostate::GoState;
use crate::mctsalgo::MctsAlgo;
use crate::state::State;
use crate::tree::Mcts;

mod mctsalgo;
mod tree;
mod state;
mod gostate;


pub fn main() {
    let mcts: Mcts<GoState> = Mcts::new();

    println!("{:?}", mcts);

    let root = mcts.root();
    mcts.play(root);

    // let last = mcts.explore(root, 5);


    // println!("{:?}", root);
    // println!("{:?}", last);

    println!("{:?}", mcts);


    // println!("{:?}", mcts.ancestor_count(&root));
    // println!("{:?}", mcts.ancestor_count(&last));
}

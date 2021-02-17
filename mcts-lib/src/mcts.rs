use crate::state::State;

pub trait Mcts<A> {
    fn new(seed: u64) -> Self;

    fn best_play<S>(&self, state: &S) -> A where S: State<A>;

    fn explore<S>(&mut self, state: &mut S) where S: State<A>;
}

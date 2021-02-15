use crate::state::State;

pub trait Mcts<A, S> where S: State<A> {
    fn new() -> Self;

    fn best_play(&self, state: &S) -> A;

    fn explore(&self, state: &mut S);
}

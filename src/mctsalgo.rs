use crate::state::{GameResult, State};

pub trait MctsAlgo<A, S>
    where S: State<A> {
    fn new() -> Self;

    fn root(&self) -> &S;

    // simulate a game
    fn play(&self, first: &S) -> (GameResult, Vec<S>);
}

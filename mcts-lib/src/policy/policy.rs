use state::{State, Action};

pub trait Policy<A: Action, S: State<A>> {
    fn select(&self, state: &S) -> A;
}

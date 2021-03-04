use rules::{Rules, Action};

pub trait Policy<A: Action, S: Rules<A>> {
    fn select(&self, state: &S) -> A;
}

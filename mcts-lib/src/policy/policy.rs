use state::State;

pub trait Policy<A, S> where S: State<A> {
    fn select(&self, state: &S) -> A;
}

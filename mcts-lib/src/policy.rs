use state::State;

pub trait Policy<A> {
    fn select<S: State<A>>(&self, state: &S) -> A;
}

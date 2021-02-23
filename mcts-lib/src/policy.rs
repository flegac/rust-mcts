use state::State;

pub trait Policy<A: Copy> {
    fn select<S: State<A>>(&self, state: &S) -> A;
}

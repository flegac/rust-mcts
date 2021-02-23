use policy::Policy;

use crate::state::State;

pub trait Mcts<A: Copy> {
    fn explore<S: State<A>>(&mut self, state: &mut S);
}

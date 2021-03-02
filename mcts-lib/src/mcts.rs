use std::hash::Hash;

use policy::policy::Policy;
use policy::score::Score;
use sim_result::SimResult;
use state::State;

pub trait Action: Copy + Eq + Hash {}

impl<T: Copy + Eq + Hash> Action for T {}

pub trait MState<A, S> {
    fn apply_action(&mut self, a: A);
    fn state(&self) -> &S;
    fn state_mut(&mut self) -> &mut S;
    fn depth(&self) -> usize;

    fn is_selectable(&self) -> bool;
    fn is_terminal(&self) -> bool;
}

pub trait Mcts<A, S, SS>
where
    S: State<A>,
    SS: MState<A, S>,
{
    fn selection<Sc: Score>(&self, state: &mut SS, exploitation: &Sc);
    fn expansion<P: Policy<A, S>>(&self, state: &mut SS, policy: &P) -> A;
    fn simulation<P: Policy<A, S>>(&self, state: &mut SS, policy: &P) -> SimResult;
    fn backpropagation(&self, state: &mut SS, res: SimResult);
}

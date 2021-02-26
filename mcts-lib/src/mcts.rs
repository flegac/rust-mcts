use graph_lib::safe_tree::Tree;
use policy::Policy;
use sim_result::SimResult;
use state::State;

pub trait MState<A, S> {
    fn apply_action(&mut self, a: A);
    fn state(&self) -> &S;
    fn state_mut(&mut self) -> &mut S;
    fn depth(&self) -> usize;
}

pub trait Mcts<A, S, SS>
    where
        SS: MState<A, S>
{
    fn selection(&self, state: &mut SS);
    fn expansion<P: Policy<A>>(&self, state: &mut SS, policy: &P) -> A;
    fn simulation<P: Policy<A>>(&self, state: &mut SS, policy: &P) -> SimResult;
    fn backpropagation(&self, state: &mut SS, res: SimResult);
}


use std::hash::Hash;

use graph_lib::safe_tree::Tree;
use policy::policy::Policy;
use policy::score::Score;
use sim_result::SimResult;
use rules::{Action, Rules};

pub(crate) type MctsNode<A> = Tree<A, SimResult>;

pub trait Mcts<A: Action, S: Rules<A>> {
    fn root(&self) -> MctsNode<A>;
    fn selection<Sc: Score>(&mut self, exploitation: &Sc) -> MctsNode<A>;
    fn expansion<P: Policy<A, S>>(&mut self, selected: &MctsNode<A>, policy: &P) -> (A, MctsNode<A>);
    fn backpropagation(&mut self, cursor: &MctsNode<A>, res: SimResult);

    fn state(&self) -> &S;
    fn state_mut(&mut self) -> &mut S;
}

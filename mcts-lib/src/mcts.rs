use policy::Policy;

use crate::state::State;

pub trait Mcts {
    fn selection(&mut self);
    fn expansion(&mut self);
    fn simulation(&mut self);
    fn backpropagation(&mut self);
}

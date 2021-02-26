use std::fmt::{Display, Formatter};
use std::fmt;
use std::hash::Hash;

use graph_lib::tree::TheTree;
use mcts::{MState, Action};
use sim_result::{MctsNode, SimResult};
use state::State;

pub struct MyState<A, S>
    where
        A: Eq,
        A: Hash,
        S: State<A> {
    the_state: S,
    pub(crate) node: MctsNode<A>,
    depth: usize,
}


impl<A, S> MyState<A, S>
    where
        A: Action,
        S: State<A> {
    pub fn new(state: S, node: MctsNode<A>) -> MyState<A, S> {
        MyState {
            the_state: state,
            node,
            depth: 0,
        }
    }
    pub(crate) fn get_node(&self) -> MctsNode<A> {
        self.node.clone()
    }
    pub(crate) fn setup_node(&mut self, root: MctsNode<A>) {
        self.node = root.clone();
        self.the_state.reset();
        self.depth = 0;

        let parents = self.node.parents();
        let n = parents.len();
        for (action, _value) in parents.iter().rev() {
            self.apply_action(action.clone())
        }

        if !parents.is_empty() {
            log::debug!("{} parents", parents.len());
        } else {
            self.extend_node()
        }
    }
    pub(crate) fn move_to_node(&mut self, action: A, node: MctsNode<A>) {
        self.node.set_child(action, &node);
        self.node = node;
        self.apply_action(action);
    }

    fn extend_node(&mut self) {
        let actions = self.the_state.actions();
        let n = actions.len();
        if self.node.children.borrow().is_empty() {
            for (i, &a) in actions.iter().enumerate() {
                let next_current = SimResult::node();
                self.node.set_child(a, &next_current);
            }
        }
    }
}

impl<A, S> Display for MyState<A, S>
    where
        A: Action,
        S: State<A>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(depth={}) {}", self.depth, self.node)
    }
}

impl<A, S> MState<A, S> for MyState<A, S>
    where
        A: Action,
        S: State<A>
{
    fn apply_action(&mut self, a: A) {
        self.the_state.apply(a);
        self.depth += 1;
    }

    fn state(&self) -> &S {
        &self.the_state
    }
    fn state_mut(&mut self) -> &mut S {
        &mut self.the_state
    }


    fn depth(&self) -> usize {
        self.depth
    }

    fn is_selectable(&self) -> bool {
        !self.get_node().value.borrow().is_leaf()
    }

    fn is_terminal(&self) -> bool {
        self.state().result().is_some()
    }
}

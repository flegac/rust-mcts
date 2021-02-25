use std::fmt::{Display, Formatter};
use std::fmt;
use std::hash::Hash;

use action_stats::ActionStats;
use mcts::{MctsNode, MState};
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
        A: Copy,
        A: Eq,
        A: Hash,
        S: State<A> {
    pub fn new(state: S, node: MctsNode<A>) -> MyState<A, S> {
        MyState {
            the_state: state,
            node,
            depth: 0,
        }
    }
    pub(crate) fn node(&self) -> MctsNode<A> {
        self.node.clone()
    }
    pub(crate) fn setup_node(&mut self, root: MctsNode<A>) {
        self.node = root;
        self.the_state.reset();
        self.depth = 0;

        let parents = self.node.parents();
        for n in parents.iter().rev() {
            match n.value.borrow().action {
                None => {}
                Some(action) => self.apply_action(action)
            }
        }
        let last_action = self.node.value.borrow().action;
        match last_action {
            None => {}
            Some(action) => self.apply_action(action)
        }
        if !parents.is_empty() {
            log::debug!("{} parents", parents.len());
        } else {
            self.extend_node()
        }
    }
    pub(crate) fn add_node(&mut self, node: MctsNode<A>) {
        let a = node.value.borrow().action.unwrap();
        self.node = node;
        self.apply_action(a);
    }

    fn extend_node(&mut self) {
        let actions = self.the_state.actions();

        if self.node.children.borrow().is_empty() {
            for a in actions {
                let next_current = ActionStats::node(Some(a));
                self.node.set_child(a, &next_current);
            }
        }
    }
}

impl<A, S> Display for MyState<A, S>
    where
        A: Display,
        A: Copy,
        A: Eq,
        A: Hash,
        S: State<A>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(depth={}) {}", self.depth, self.node)
    }
}

impl<A, S> MState<A, S> for MyState<A, S>
    where
        A: Copy,
        A: Eq,
        A: Hash,
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
}

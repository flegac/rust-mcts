use std::fmt::{Display, Formatter};
use std::fmt;

use action_stats::ActionStats;
use mcts::{MctsNode, MState};
use state::State;

pub struct MyState<A, S: State<A>> {
    state: S,
    pub(crate) node: MctsNode<A>,
    depth: usize,
}


impl<A, S: State<A>> MyState<A, S> {
    pub fn new(state: S, node: MctsNode<A>) -> MyState<A, S> {
        MyState {
            state,
            node,
            depth: 0,
        }
    }

    fn extend_node(&mut self) {
        let actions = self.state.actions();

        if self.node.children.borrow().is_empty() {
            for a in actions {
                let next_current = ActionStats::node(Some(a));
                self.node.add_child(&next_current);
            }
        }
    }
}

impl<A, S> Display for MyState<A, S>
    where A: Copy, A: Display, S: State<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(depth={}) {}", self.depth, self.node, )
    }
}

impl<A, S> MState<A, S> for MyState<A, S>
    where A: Copy, A: Display, S: State<A>, S: Display {
    fn setup_node(&mut self, root: MctsNode<A>) {
        self.node = root;
        self.state.reset();
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

    fn add_node(&mut self, node: MctsNode<A>) {
        let a = node.value.borrow().action.unwrap();
        self.node = node;
        self.apply_action(a);
    }

    fn apply_action(&mut self, a: A) {
        self.state.apply(a);
        self.depth += 1;
    }

    fn state(&self) -> &S {
        &self.state
    }
    fn state_mut(&mut self) -> &mut S {
        &mut self.state
    }

    fn node(&self) -> MctsNode<A> {
        self.node.clone()
    }

    fn depth(&self) -> usize {
        self.depth
    }
}

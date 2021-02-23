use std::fmt::{Display, Formatter};
use std::fmt;

use graph_lib::safe_tree::SafeTree;
use graph_lib::tree::Tree;
use mcts_stats::MctsStats;
use state::State;

pub struct MctsState<A, S>
    where A: Copy,
          S: State<A> {
    pub state: S,
    pub current: SafeTree<MctsStats<A>>,
    pub depth: usize,
}

impl<A, S> MctsState<A, S>
    where
        A: Copy,
        S: State<A> {
    pub(crate) fn reset(&mut self, root: &SafeTree<MctsStats<A>>) {
        self.current = root.clone();
        self.state.reset();
        self.depth = 0;
    }

    pub fn extend_node(&self, actions: Vec<A>) {
        if self.current.children.borrow().is_empty() {
            for a in actions {
                let next_current = SafeTree::new(MctsStats::new(Some(a)));
                self.current.add_child(&next_current);
            }
        }
    }
}

impl<A, S> Display for MctsState<A, S>
    where A: Copy,
          A: Display,
          S: State<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(depth={}) {}", self.depth, self.current, )
    }
}

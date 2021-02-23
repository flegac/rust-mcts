use std::borrow::Borrow;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use ordered_float::OrderedFloat;

use graph_lib::node::Node;
use graph_lib::safe_tree::SafeTree;
use graph_lib::tree::Tree;
use mcts_state::MctsState;
use policy::Policy;
use state::GameResult;

use crate::mcts::Mcts;
use crate::mcts_stats::MctsStats;
use crate::state::State;

pub struct MyMcts<A, S, P>
    where A: Copy,
          S: State<A>,
          P: Policy<A> {
    pub root: SafeTree<MctsStats<A>>,
    policy: P,
    pub state: MctsState<A, S>,
}

impl<A, S, P> MyMcts<A, S, P>
    where A: Copy,
          A: Eq,
          A: Display,
          S: State<A>,
          S: Display,
          P: Policy<A> {
    pub fn new(state: S, policy: P) -> MyMcts<A, S, P> {
        let root = SafeTree::new(MctsStats::new(None));
        MyMcts {
            root: root.clone(),
            policy,
            state: MctsState {
                current: root.clone(),
                state,
                depth: 0,
            },
        }
    }

    pub fn explore(&mut self) {
        log::debug!("* Exploration:");

        self.state.reset(&self.root);
        self.selection();
        self.expansion();
        self.simulation();
        self.backpropagation();
    }
}

impl<A, S, P> Mcts for MyMcts<A, S, P>
    where A: Copy,
          A: Display,
          S: State<A>,
          S: Display,
          P: Policy<A> {
    fn selection(&mut self) {
        let mut current = self.state.current.clone();
        let mut res = None;

        let is_leaf = |x: &MctsStats<A>| x.explored == 0;

        self.state.extend_node(self.state.state.actions());
        while !is_leaf(current.value.borrow().deref()) {
            let N = current.value.borrow().explored;
            let found = current.max_by_key(|a| a.selection_score(N));

            match found {
                None => break,
                Some(xx) => {
                    current = xx;
                    let a = current.value.borrow().action.unwrap();
                    self.state.state.apply(&a);
                    self.state.depth += 1;
                    res = self.state.state.result();
                }
            }
        }
        self.state.current = current;
        log::debug!("Selection: {}", self.state);
    }

    fn expansion(&mut self) {
        let a = self.policy.select(&self.state.state);
        let next_current = SafeTree::new(MctsStats::new(Some(a)));
        self.state.current.add_child(&next_current);
        self.state.state.apply(&a);
        self.state.current = next_current;
        self.state.depth += 1;
        log::debug!("Expansion: {}", self.state);
    }

    fn simulation(&mut self) {
        let mut res = self.state.state.result();
        while res.is_none() {
            let a = self.policy.select(&self.state.state);
            self.state.state.apply(&a);
            self.state.depth += 1;
            res = self.state.state.result();
        }

        log::debug!("Simulation: {:?}", res);
    }

    fn backpropagation(&mut self) {
        let mut result = self.state.state.result().unwrap();
        self.state.current.value.borrow_mut().update_score(result);
        for c in self.state.current.parents() {
            c.value.borrow_mut().update_score(result);
            result = result.switch();
        }
        log::debug!("Backpropagation: {:?} {}", result, self.state);
    }
}

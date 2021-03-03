use core::mem;
use std::borrow::Borrow;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

use ordered_float::OrderedFloat;

use graph_lib::algo::trees::Trees;
use graph_lib::safe_tree::Tree;
use graph_lib::tree::TheTree;
use mcts::Mcts;
use policy::policy::Policy;
use policy::score::Score;
use policy::win_score::ExploreScore;
use sim_result::SimResult;
use state::{Action, State};

use crate::mcts::MctsNode;

pub struct MyMcts<A: Action, S: State<A>> {
    the_state: S,
    root: MctsNode<A>,
}

impl<A: Action, S: State<A>> MyMcts<A, S> {
    pub fn new(state: S) -> MyMcts<A, S> {
        MyMcts {
            the_state: state,
            root: SimResult::node(),
        }
    }

    pub fn fork(&self, node: &MctsNode<A>) -> MyMcts<A, S> {
        MyMcts {
            the_state: self.state().clone(),
            root: node.clone(),
        }
    }

    fn is_leaf(node: MctsNode<A>) -> bool {
        node.value.borrow().is_leaf()
    }

    pub(crate) fn setup_node(&mut self, current: MctsNode<A>) {
        self.the_state.reset();
        for (action, _) in current.parents().iter().rev() {
            self.state_mut().apply_action(action.clone())
        }
    }

    pub(crate) fn reset(&mut self) {
        self.setup_node(self.root.clone());
    }
}

impl<A: Action, S: State<A>> Mcts<A, S> for MyMcts<A, S> {
    fn root(&self) -> MctsNode<A> {
        self.root.clone()
    }

    fn selection<Sc: Score>(&mut self, exploitation: &Sc) -> MctsNode<A> {
        self.reset();
        let mut cursor = self.root();

        while Self::is_leaf(cursor.clone()) {
            let score = |child: &SimResult| {
                let copy = cursor.clone();
                let parent = copy.value.borrow();
                OrderedFloat(exploitation.score(child) + ExploreScore::new(&parent).score(child))
            };
            match cursor.search_max_child(&score) {
                None => break,
                Some((action, node)) => {
                    cursor.set_child(action, &node);
                    self.state_mut().apply_action(action);
                    cursor = node.clone();
                }
            }
        }
        cursor
    }

    fn expansion<P: Policy<A, S>>(&mut self, selected: &MctsNode<A>, policy: &P) -> (A, MctsNode<A>) {
        let action = policy.select(self.state());
        let new_node = SimResult::node();
        selected.set_child(action, &new_node);
        self.state_mut().apply_action(action);
        (action, new_node)
    }


    fn backpropagation(&mut self, cursor: &MctsNode<A>, mut res: SimResult) {
        cursor.value.borrow_mut().merge(&res);
        let parents = cursor.parents();
        for (_key, value) in parents {
            value.value.borrow_mut().merge(&res);
            res.swap();
        }
    }

    fn state(&self) -> &S {
        &self.the_state
    }

    fn state_mut(&mut self) -> &mut S {
        &mut self.the_state
    }
}

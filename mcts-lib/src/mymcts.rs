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
    depth: usize,
}

fn is_leaf<A: Action>(node: MctsNode<A>) -> bool {
    node.value.borrow().is_leaf()
}

impl<A: Action, S: State<A>> MyMcts<A, S> {
    pub fn new(state: S) -> MyMcts<A, S> {
        MyMcts {
            the_state: state,
            root: SimResult::node(),
            depth: 0,
        }
    }


    pub(crate) fn setup_node(&mut self, current: MctsNode<A>) {
        self.the_state.reset();
        self.depth = 0;

        let parents = current.parents();
        for (action, _value) in parents.iter().rev() {
            self.state_mut().apply(action.clone())
        }

        if !parents.is_empty() {
            log::debug!("{} parents", parents.len());
        } else {
            self.extend_node(&current)
        }
    }

    pub(crate) fn reset(&mut self) {
        self.setup_node(self.root.clone());
    }

    pub(crate) fn move_to_node(&mut self, action: A, cursor: &MctsNode<A>, node: &MctsNode<A>) {
        cursor.set_child(action, &node);
        self.state_mut().apply(action);
    }

    fn extend_node(&mut self, node: &MctsNode<A>) {
        let actions = self.the_state.actions();
        if node.children.borrow().is_empty() {
            for (_i, &a) in actions.iter().enumerate() {
                let next_current = SimResult::node();
                node.set_child(a, &next_current);
            }
        }
    }
}

impl<A, S> Display for MyMcts<A, S>
    where
        A: Action,
        S: State<A>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(depth={})", self.depth)
    }
}

impl<A: Action, S: State<A>> Mcts<A, S> for MyMcts<A, S> {
    fn root(&self) -> MctsNode<A> {
        self.root.clone()
    }

    fn selection<Sc: Score>(&mut self, exploitation: &Sc) -> MctsNode<A> {
        self.reset();
        let mut cursor = self.root();

        while is_leaf(cursor.clone()) {
            let copy = cursor.clone();
            let parent = copy.value.borrow();
            let score = |child: &SimResult| {
                OrderedFloat(exploitation.score(child) + ExploreScore::new(&parent).score(child))
            };
            let found = cursor.search_max_child(&score);
            match found {
                None => break,
                Some((action, node)) => {
                    cursor.set_child(action, &node);
                    self.state_mut().apply(action);
                    // mem::swap(&mut cursor, &mut node);
                    cursor = node.clone();
                }
            }
        }
        cursor
    }
    fn expansion<P: Policy<A, S>>(&mut self, cursor: &MctsNode<A>, policy: &P) -> (A, MctsNode<A>) {
        let action = policy.select(self.state());

        let new_node = SimResult::node();

        cursor.set_child(action, &new_node);
        self.state_mut().apply(action);
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

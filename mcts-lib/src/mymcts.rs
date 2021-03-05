use core::mem;
use std::borrow::Borrow;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::ops::Deref;

use ordered_float::OrderedFloat;

use graph_lib::algo::trees::Trees;
use graph_lib::safe_tree::Tree;
use graph_lib::tree::TheTree;
use mcts::Mcts;
use policy::policy::Policy;
use policy::score::Score;
use policy::win_score::ExploreScore;
use rules::{Action, Rules};
use sim_result::SimResult;

use crate::mcts::MctsNode;

pub struct MyMcts<A: Action, S: Rules<A>> {
    the_state: S,
    root: MctsNode<A>,
}

impl<A: Action, S: Rules<A>> MyMcts<A, S> {
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


fn selection_score<'a, A: 'a + Action, Sc: Score>(
    cursor: &MctsNode<A>,
    exploitation: &'a Sc,
) -> impl Fn(&SimResult) -> OrderedFloat<f32> + 'a
{
    let copy = cursor.clone();
    move |child: &SimResult| {
        let parent = copy.value.borrow();
        OrderedFloat(exploitation.score(child) + ExploreScore::new(&parent).score(child))
    }
}

impl<A: Action, S: Rules<A>> Mcts<A, S> for MyMcts<A, S> {
    fn root(&self) -> MctsNode<A> {
        self.root.clone()
    }

    fn selection<Sc: Score>(&mut self, exploitation: &Sc) -> MctsNode<A> {
        self.reset();
        let mut cursor = self.root();
        let mut final_score = OrderedFloat(0_f32);
        while !Self::is_leaf(cursor.clone()) {
            let score = selection_score(&cursor, exploitation);
            match cursor.search_max_child(&score) {
                None => break,
                Some((action, node)) => {
                    final_score = score(node.value.borrow().deref());
                    log::debug!("depth={}, score={}", node.depth.borrow(), final_score);
                    cursor.set_child(action, &node);
                    self.state_mut().apply_action(action);
                    cursor = node.clone();
                }
            }
        }
        log::debug!("Selection: depth={:?} score={}", cursor.depth.borrow(), final_score);
        cursor
    }

    fn expansion<P: Policy<A, S>>(&mut self, selected: &MctsNode<A>, policy: &P) -> (A, MctsNode<A>) {
        let action = policy.select(self.state());
        self.state_mut().apply_action(action);

        let mut next_node = selected.clone();
        for a in self.state().actions() {
            let new_node = SimResult::node();
            selected.set_child(a, &new_node);
            if a == action {
                next_node = new_node;
            }
        }
        log::debug!("Expansion: {:?}\n{}", action, next_node);
        (action, next_node)
    }


    fn backpropagation(&mut self, cursor: &MctsNode<A>, mut res: SimResult) {
        log::debug!("Backpropagation: ({} parents)", cursor.parents().len());
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

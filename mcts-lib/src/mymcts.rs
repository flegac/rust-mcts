use std::fmt::Display;
use std::hash::Hash;

use ordered_float::OrderedFloat;

use mcts::{MctsNode, MState};
use mystate::MyState;
use policy::Policy;
use sim_result::SimResult;
use state::GameResult;

use crate::action_stats::ActionStats;
use crate::mcts::Mcts;
use crate::state::State;

pub struct MyMcts<A, S, SS>
    where A: Copy,
          A: Eq,
          A: Hash,
          SS: MState<A, S>
{
    pub root: MctsNode<A>,
    simulation_factor: usize,
    _foo: Option<(S, SS)>,
}


impl<A, S> MyMcts<A, S, MyState<A, S>>
    where
        A: Display,
        A: Copy,
        A: Eq,
        A: Hash,
        S: State<A>,
{
    pub fn new(simulation_factor: usize) -> MyMcts<A, S, MyState<A, S>> {
        MyMcts {
            root: ActionStats::node(),
            simulation_factor,
            _foo: None,
        }
    }
    pub fn get_state(&self, state: S) -> MyState<A, S> {
        MyState::new(state, self.root.clone())
    }

    fn sim_once<P: Policy<A>>(&self, state: &mut MyState<A, S>, policy: &P) -> GameResult {
        let mut res = state.state().result();
        while state.state().result().is_none() {
            let a = policy.select(state.state());
            state.apply_action(a);
            res = state.state().result();
        }
        res.unwrap()
    }

    pub fn explore<P: Policy<A>>(&mut self, state: &mut MyState<A, S>, policy: &P) {
        log::debug!("* Exploration:");
        state.setup_node(self.root.clone());

        self.selection(state);
        let selection_depth = state.depth();
        log::debug!("Selection: depth={}", selection_depth);
        let action = self.expansion(state, policy);
        log::debug!("Expansion: {}", action);

        // let before_score = <MyMcts<A, S>>::compute_score(state);
        let res = self.simulation(state, policy);
        log::debug!("Simulation: {}", res);
        self.backpropagation(state, res);
        let n = state.node().parents().len();
        log::debug!("Backpropagation: ({} parents)", n);
        // let after_score = <MyMcts<A, S>>::compute_score(state);
        // log::debug!("Exploration: from depth {}, score: {} -> {}",
        //            selection_depth,
        //            before_score,
        //            after_score)
    }
}

impl<A, S> Mcts<A, S, MyState<A, S>> for MyMcts<A, S, MyState<A, S>>
    where
        A: Display,
        A: Copy,
        A: Eq,
        A: Hash,
        S: State<A>,
{
    fn selection(&self, state: &mut MyState<A, S>) {
        while !state.node().value.borrow().is_leaf() {
            let found = state.node().max_by_key(
                |a| a.score(&state.node().value.borrow()));

            // target = SafeTree<ActionStats<A>>

            match found {
                None => break,
                Some((action, node)) => {
                    state.add_node(action, node);
                }
            }
        }
    }

    fn expansion<P: Policy<A>>(&self, state: &mut MyState<A, S>, policy: &P) -> A {
        let action = policy.select(state.state());
        let next_current = ActionStats::node();
        state.node().set_child(action, &next_current);
        state.add_node(action, next_current);
        action
    }

    fn simulation<P: Policy<A>>(&self, state: &mut MyState<A, S>, policy: &P) -> SimResult {
        let mut res = SimResult::new();
        match self.simulation_factor {
            1 => {
                res.update(self.sim_once(state, policy));
            }
            _ => {
                let node = state.node().clone();
                for i in 0..self.simulation_factor {
                    println!("sim #{}", i);
                    let the_node = node.clone();
                    state.setup_node(the_node);
                    res.update(self.sim_once(state, policy));
                }
            }
        }
        res
    }


    fn backpropagation(&self, state: &mut MyState<A, S>, mut res: SimResult) {
        state.node().value.borrow_mut().stats.merge(&res);
        let parents = state.node().parents();
        for (key, value) in parents {
            value.value.borrow_mut().stats.merge(&res);
            res.swap();
        }
    }
}

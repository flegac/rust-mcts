use std::fmt::Display;

use ordered_float::OrderedFloat;

use graph_lib::tree::Tree;
use mcts::{MctsNode, MState};
use mystate::MyState;
use policy::Policy;
use sim_result::SimResult;
use state::GameResult;

use crate::action_stats::ActionStats;
use crate::mcts::Mcts;
use crate::state::State;

pub struct MyMcts<A: Copy, S, SS> {
    pub root: MctsNode<A>,
    simulation_factor: usize,
    _foo: Option<(S, SS)>,
}


impl<A, S, SS> MyMcts<A, S, SS>
    where
        A: Copy,
        A: Eq,
        S: State<A>,
        S: Display,
        SS: MState<A, S>,
{
    pub fn new(simulation_factor: usize) -> MyMcts<A, S, SS> {
        let root = ActionStats::node(None);
        MyMcts {
            root: root.clone(),
            simulation_factor,
            _foo: None,
        }
    }
    pub fn get_state(&self, state: S) -> MyState<A, S> {
        MyState::new(state, self.root.clone())
    }

    fn sim_once<P: Policy<A>>(&self, state: &mut SS, policy: &P) -> GameResult {
        let mut res = state.state().result();
        while state.state().result().is_none() {
            let a = policy.select(state.state());
            state.apply_action(a);
            res = state.state().result();
        }
        res.unwrap()
    }

    pub fn explore<P: Policy<A>>(&mut self, state: &mut SS, policy: &P) {
        log::trace!("* Exploration:");
        state.setup_node(self.root.clone());

        self.selection(state);
        let selection_depth = state.depth();
        log::trace!("Selection: depth={}", selection_depth);
        self.expansion(state, policy);
        log::trace!("Expansion: {}", state.state());

        // let before_score = <MyMcts<A, S>>::compute_score(state);
        let res = self.simulation(state, policy);
        log::trace!("Simulation: {}", res);
        self.backpropagation(state, res);
        let n = state.node().parents().len();
        log::trace!("Backpropagation: ({} parents) {}", n, state.state());
        // let after_score = <MyMcts<A, S>>::compute_score(state);
        // log::debug!("Exploration: from depth {}, score: {} -> {}",
        //            selection_depth,
        //            before_score,
        //            after_score)
    }
}

impl<A, S, SS> Mcts<A, S, SS> for MyMcts<A, S, SS>
    where
        A: Copy,
        A: Eq,
        S: State<A>,
        S: Display,
        SS: MState<A, S>
{
    fn selection(&self, state: &mut SS) {
        while !state.node().value.borrow().is_leaf() {
            let found = state.node().max_by_key(
                |a| a.score(&state.node().value.borrow()));

            // target = SafeTree<ActionStats<A>>

            match found {
                None => break,
                Some(xx) => {
                    let current = xx;
                    state.add_node(current);
                }
            }
        }
    }

    fn expansion<P: Policy<A>>(&self, state: &mut SS, policy: &P) {
        let a = policy.select(state.state());
        let next_current = ActionStats::node(Some(a));
        state.node().add_child(&next_current);
        state.add_node(next_current);
    }

    fn simulation<P: Policy<A>>(&self, state: &mut SS, policy: &P) -> SimResult {
        let mut res = SimResult::new();
        match self.simulation_factor {
            1 => {
                res.update(self.sim_once(state, policy));
            }
            _ => {
                let node = state.node().clone();
                for _ in 0..self.simulation_factor {
                    state.setup_node(node.clone());
                    res.update(self.sim_once(state, policy));
                }
            }
        }
        res
    }


    fn backpropagation(&self, state: &mut SS, mut res: SimResult) {
        state.node().value.borrow_mut().stats.merge(&res);
        let parents = state.node().parents();
        for c in parents {
            c.value.borrow_mut().stats.merge(&res);
            res.swap();
        }
    }
}

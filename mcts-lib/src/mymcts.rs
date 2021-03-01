use std::fmt::Display;

use ordered_float::OrderedFloat;

use graph_lib::algo::trees::Trees;
use mcts::{Action, MState};
use mystate::MyState;
use policy::policy::Policy;
use policy::score::Score;
use policy::win_score::ExploreScore;
use sim_result::{MctsNode, SimResult};

use crate::mcts::Mcts;
use crate::state::State;

pub struct MyMcts<A, S, SS>
    where
        A: Action,
        SS: MState<A, S>,
{
    pub root: MctsNode<A>,
    simulation_factor: usize,
    _foo: Option<(S, SS)>,
}

impl<A, S> MyMcts<A, S, MyState<A, S>>
    where
        A: Action,
        A: Display,
        S: State<A>,
{
    pub fn new(simulation_factor: usize) -> MyMcts<A, S, MyState<A, S>> {
        MyMcts {
            root: SimResult::node(),
            simulation_factor,
            _foo: None,
        }
    }
    pub fn get_state(&self, state: S) -> MyState<A, S> {
        MyState::new(state, self.root.clone())
    }

    pub fn explore<Sim, Select>(
        &mut self,
        state: &mut MyState<A, S>,
        sim_policy: &Sim,
        select_policy: &Select,
    ) where
        Sim: Policy<A, S>,
        Select: Score,
    {
        log::debug!("* Exploration:");
        state.setup_node(self.root.clone());

        self.selection(state, select_policy);
        let selection_depth = state.depth();
        log::debug!("Selection: depth={}", selection_depth);
        let action = self.expansion(state, sim_policy);
        log::debug!("Expansion: {}", action);

        // let before_score = <MyMcts<A, S>>::compute_score(state);
        let res = self.sim_many(state, sim_policy);
        log::debug!("Simulation: {}", res);
        self.backpropagation(state, res);
        let n = state.get_node().parents().len();
        log::debug!("Backpropagation: ({} parents)", n);
        // let after_score = <MyMcts<A, S>>::compute_score(state);
        // log::debug!("Exploration: from depth {}, score: {} -> {}",
        //            selection_depth,
        //            before_score,
        //            after_score)
    }
    fn sim_many<Sim: Policy<A, S>>(&self, state: &mut MyState<A, S>, policy: &Sim) -> SimResult {
        match self.simulation_factor {
            1 => self.simulation(state, policy),
            _ => {
                let mut res = SimResult::new();
                let node = state.get_node().clone();
                for _i in 0..self.simulation_factor {
                    let the_node = node.clone();
                    state.setup_node(the_node);
                    res.merge(&self.simulation(state, policy));
                }
                res
            }
        }
    }
}

impl<A, S> Mcts<A, S, MyState<A, S>> for MyMcts<A, S, MyState<A, S>>
    where
        A: Action,
        S: State<A>,
{
    fn selection<Sc: Score>(&self, state: &mut MyState<A, S>, exploitation: &Sc) {
        while state.is_selectable() {
            let node = state.get_node();
            let parent = node.value.borrow();

            let score = |child: &SimResult| {
                OrderedFloat(exploitation.score(child) + ExploreScore::new(&parent).score(child))
            };
            let found = node.search_max_child(&score);
            match found {
                None => break,
                Some((action, node)) => {
                    state.move_to_node(action, node);
                }
            }
        }
    }

    fn expansion<P: Policy<A, S>>(&self, state: &mut MyState<A, S>, policy: &P) -> A {
        let action = policy.select(state.state());
        state.move_to_node(action, SimResult::node());
        action
    }

    fn simulation<P: Policy<A, S>>(&self, state: &mut MyState<A, S>, policy: &P) -> SimResult {
        while !state.is_terminal() {
            let action = policy.select(state.state());
            state.apply_action(action);
        }
        SimResult::from_game(state.state().result().unwrap())
    }

    fn backpropagation(&self, state: &mut MyState<A, S>, mut res: SimResult) {
        state.get_node().value.borrow_mut().merge(&res);
        let parents = state.get_node().parents();
        for (_key, value) in parents {
            value.value.borrow_mut().merge(&res);
            res.swap();
        }
    }
}

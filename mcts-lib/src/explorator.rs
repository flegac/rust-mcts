use std::borrow::Borrow;
use std::fmt::Display;
use std::ops::Deref;

use ordered_float::OrderedFloat;

use graph_lib::algo::trees::Trees;
use mymcts::MyMcts;
use policy::policy::Policy;
use policy::score::Score;
use policy::win_score::ExploreScore;
use sim_result::SimResult;
use state::Action;

use crate::mcts::{Mcts, MctsNode};
use crate::state::State;

pub struct Explorator<A: Action, S: State<A>> {
    mcts: MyMcts<A, S>,
    simulation_factor: usize,
    _foo: Option<(S)>,
}

impl<A: Action + Display, S: State<A>> Explorator<A, S> {
    pub fn new(simulation_factor: usize, state: S) -> Explorator<A, S> {
        Explorator {
            mcts: MyMcts::new(state),
            simulation_factor,
            _foo: None,
        }
    }

    pub fn mcts(&self) -> &MyMcts<A, S> {
        &self.mcts
    }

    pub fn mcts_mut(&mut self) -> &mut MyMcts<A, S> {
        &mut self.mcts
    }

    pub fn explore<Sim: Policy<A, S>, Select: Score>(&mut self,
                                                     sim_policy: &Sim,
                                                     select_policy: &Select) -> MctsNode<A> {
        log::debug!("* Exploration:");
        let selected = self.mcts.selection( select_policy);
        log::debug!("Selection: depth={:?}", &selected.depth);

        let (action, expansion) = self.mcts.expansion(&selected, sim_policy);
        log::debug!("Expansion: {}", action);

        let res = self.simulation(sim_policy);
        log::debug!("Simulation: {}", res);

        log::debug!("Backpropagation: ({} parents)", expansion.parents().len());
        self.mcts.backpropagation(&expansion, res);

        expansion
    }

    fn simulation<Sim: Policy<A, S>>(&mut self, policy: &Sim) -> SimResult {
        match self.simulation_factor {
            1 => self.mcts.state_mut().simulation(policy),
            _ => {
                let mut result = SimResult::new();
                for _i in 0..self.simulation_factor {
                    let res = self.mcts.state().clone().simulation(policy);
                    result.merge(&res);
                }
                result
            }
        }
    }
}


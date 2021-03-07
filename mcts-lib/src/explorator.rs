use std::borrow::Borrow;
use std::fmt::Display;
use std::ops::Deref;

use ordered_float::OrderedFloat;

use graph_lib::algo::trees::Trees;
use mymcts::MyMcts;
use policy::policy::Policy;
use policy::score::Score;
use policy::win_score::ExploreScore;
use rust_tools::screen::layout::layout::L;
use sim_result::SimResult;
use rules::Action;

use crate::mcts::{Mcts, MctsNode};
use crate::rules::Rules;

pub struct Explorer<A: Action, S: Rules<A>> {
    mcts: MyMcts<A, S>,
    simulation_factor: usize,
    _foo: Option<(S)>,
}

impl<A: Action, S: Rules<A>> Explorer<A, S> {
    pub fn new(simulation_factor: usize, state: S) -> Explorer<A, S> {
        Explorer {
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

    pub fn explore<Sim: Policy<A, S>, Select: Score>(
        &mut self,
        sim_policy: &Sim,
        select_policy: &Select)
        -> MctsNode<A>
    {
        log::debug!("* Exploration:");
        let selected = self.mcts.selection(select_policy);
        let (_action, expansion) = self.mcts.expansion(&selected, sim_policy);
        let res = self.simulation(sim_policy);
        self.mcts.backpropagation(&expansion, res);

        expansion
    }

    fn simulation<Sim: Policy<A, S>>(&mut self, policy: &Sim) -> SimResult {
        let res = match self.simulation_factor {
            1 => self.mcts.state_mut().simulation(policy),
            _ => {
                let mut result = SimResult::new();
                let mut state = self.mcts.state().fork();
                for _i in 0..self.simulation_factor {
                    result.merge(&state.fork().simulation(policy));
                }
                result
            }
        };
        log::debug!("Simulation: {}", res);
        res
    }
}


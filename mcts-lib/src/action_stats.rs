use std::fmt;
use std::fmt::{Display, Formatter};

use ordered_float::OrderedFloat;

use graph_lib::safe_tree::Tree;
use mcts::MctsNode;
use sim_result::SimResult;

pub struct ActionStats<A> {
    pub action: Option<A>,
    pub stats: SimResult,
}

impl<A> ActionStats<A> {
    pub fn node(action: Option<A>) -> MctsNode<A> {
        Tree::new(ActionStats::new(action))
    }

    pub(crate) fn new(action: Option<A>) -> Self {
        ActionStats {
            action,
            stats: SimResult::new(),
        }
    }
    pub fn is_leaf(&self) -> bool {
        self.stats.tries == 0
    }

    pub fn score(&self, parent: &Self) -> OrderedFloat<f32> {
        let xxx = (parent.stats.tries as f32).ln();

        let x = match self.stats.tries {
            0 => xxx.sqrt(),
            n => {
                let w = self.stats.wins as f32;
                let exploitation = w / n as f32;
                let exploration = (2. * xxx / n as f32).sqrt();
                exploitation + exploration
            }
        };
        OrderedFloat(x)
    }
}

impl<A> fmt::Display for ActionStats<A> where
    A: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut action = String::new();
        match &self.action {
            None => action.push_str("pass"),
            Some(a) => action.push_str(&a.to_string())
        }
        write!(f, "[{} | {}]", action, self.stats)
    }
}
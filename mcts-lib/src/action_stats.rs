use std::fmt;
use std::fmt::Formatter;
use std::hash::Hash;

use ordered_float::OrderedFloat;

use graph_lib::safe_tree::Tree;
use mcts::MctsNode;
use sim_result::SimResult;

pub struct ActionStats {
    pub stats: SimResult,
}

impl ActionStats {
    pub fn node<A>() -> MctsNode<A>
        where
            A: Copy, A: Eq, A: Hash {
        Tree::new(ActionStats::new())
    }

    pub(crate) fn new() -> Self {
        ActionStats {
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

impl fmt::Display for ActionStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.stats)
    }
}
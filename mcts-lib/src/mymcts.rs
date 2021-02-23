use std::cell::RefCell;
use std::fmt::Display;
use std::ops::DerefMut;

use graph_lib::safe_tree::SafeTree;
use graph_lib::tree::Tree;
use policy::Policy;
use state::GameResult;

use crate::mcts::Mcts;
use crate::state::State;
use crate::stats::MctsStats;

pub struct MyMcts<A: Copy, P: Policy<A>> {
    pub root: SafeTree<MctsStats<A>>,
    current: SafeTree<MctsStats<A>>,
    policy: P,
}

impl<A: Copy, P: Policy<A>> MyMcts<A, P> {
    pub fn new(policy: P) -> MyMcts<A, P> {
        let root = SafeTree::new(MctsStats::new(None));
        MyMcts {
            root: root.clone(),
            current: root.clone(),
            policy,
        }
    }
    fn update_score(&mut self, result: GameResult) {
        for c in self.current.parents() {
            c.value.borrow_mut().explored += 1;
            match result {
                GameResult::Victory => c.value.borrow_mut().wins += 1,
                GameResult::Defeat => {}
                GameResult::Draw => c.value.borrow_mut().draws += 1
            }
        }
    }
}

impl<A, P: Policy<A>> Mcts<A> for MyMcts<A, P>
    where A: Copy, A: Display {
    fn explore<S: State<A>>(&mut self, state: &mut S) {
        let mut res = state.result();
        while res.is_none() {
            let a = self.policy.select(state);
            let next_current = SafeTree::new(MctsStats::new(Some(a)));
            self.current.add_child(&next_current);

            state.apply(&a);
            self.current = next_current;
            res = state.result();
        }

        let result = res.unwrap();
        log::debug!("{:?}", result);
        self.update_score(result);

        self.current = self.root.clone();
    }
}

use std::cell::RefCell;
use std::fmt::Display;
use std::ops::DerefMut;

use rand::prelude::SliceRandom;
use rand::SeedableRng;
use rand_pcg::Pcg64;

use state::GameResult;
use tree_lib::safe_tree::SafeTree;
use tree_lib::tree::Tree;

use crate::mcts::Mcts;
use crate::state::State;
use crate::stats::MctsStats;

pub struct MyMcts<A> {
    pub root: SafeTree<MctsStats<A>>,
    current: SafeTree<MctsStats<A>>,
    rng: RefCell<Pcg64>,
    _oups: Option<A>,
}

impl<A> MyMcts<A> {
    pub fn find_node() {
        unimplemented!()
    }
}


impl<A> Mcts<A> for MyMcts<A>
    where A: Copy, A: Display {
    fn new(seed: u64) -> MyMcts<A> {
        let root = SafeTree::new(MctsStats::new(None));
        MyMcts {
            root: root.clone(),
            current: root.clone(),
            rng: RefCell::new(Pcg64::seed_from_u64(seed)),
            _oups: None,
        }
    }


    fn best_play<S>(&self, state: &S) -> A
        where S: State<A> {
        let mut actions = state.actions();
        let mut rng = self.rng.borrow_mut();

        //TODO: sort by best expected result & exploration ratio

        actions.shuffle(rng.deref_mut());
        actions.get(0).unwrap().clone()
    }

    fn explore<S>(&mut self, state: &mut S)
        where S: State<A> {
        let mut res = state.result();
        while res.is_none() {
            let a = self.best_play(state);
            let next_current = SafeTree::new(MctsStats::new(Some(a)));
            self.current.add_child(&next_current);

            state.next(&a);
            self.current = next_current;
            res = state.result();
        }

        let result = res.unwrap();
        log::debug!("{:?}", result);

        for c in self.current.parents() {
            c.value.borrow_mut().explored += 1;
            match result {
                GameResult::Victory => c.value.borrow_mut().wins += 1,
                GameResult::Defeat => {}
                GameResult::Draw => c.value.borrow_mut().draws += 1
            }
        }

        self.current = self.root.clone();
    }
}

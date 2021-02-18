use std::cell::RefCell;
use std::fmt::Display;
use std::ops::DerefMut;

use rand::prelude::SliceRandom;
use rand::SeedableRng;
use rand_pcg::Pcg64;

use state::GameResult;
use tree_lib::tree::Tree;

use crate::mcts::Mcts;
use crate::state::State;
use crate::stats::MctsStats;

pub struct MyMcts<A> {
    pub root: Tree<MctsStats<A>>,
    current: Tree<MctsStats<A>>,
    rng: RefCell<Pcg64>,
    _oups: Option<A>,
}

impl<A> MyMcts<A> {
    pub(crate) fn find_node() {
        unimplemented!()
    }
}


impl<A> Mcts<A> for MyMcts<A>
    where A: Copy, A: Display {
    fn new(seed: u64) -> MyMcts<A> {
        let root = Tree::new(MctsStats::new(None));
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
        while state.result().is_none() {
            let a = self.best_play(state);
            let next_current = Tree::new(MctsStats::new(Some(a)));
            self.current.add_child(&next_current);

            state.next(&a);
            self.current = next_current;
        }

        let result = state.result().unwrap();
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

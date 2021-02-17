use std::cell::RefCell;
use std::ops::DerefMut;

use indextree::{Arena, NodeId};
use rand::prelude::SliceRandom;
use rand::SeedableRng;
use rand_pcg::Pcg64;

use state::GameResult;
use trees::tree::Tree;

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
    where A: Copy {
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
        actions.shuffle(rng.deref_mut());
        actions.get(0).unwrap().clone()
    }

    fn explore<S>(&mut self, state: &mut S)
        where S: State<A> {
        while state.result().is_none() {
            let a = self.best_play(state);
            self.current.add_child(&Tree::new(MctsStats::new(Some(a))));

            state.next(&a);
        }
        let result = state.result().unwrap();

        // for c in self.current.parents() {
        //     let x = c.value_mut();
        //     x.explored += 1;
        //     match result {
        //         GameResult::Victory => x.wins += 1,
        //         GameResult::Defeat => {}
        //         GameResult::Draw => x.draws += 1
        //     }
        // }
        self.current = self.root.clone();
    }
}

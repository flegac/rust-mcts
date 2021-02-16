use indextree::{Arena, NodeId};
use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::mcts::Mcts;
use crate::state::State;
use crate::stats::MctsStats;

pub struct MyMcts<A, S>
    where S: State<A> {
    arena: Arena<MctsStats<A>>,
    root_node: NodeId,
    root_state: S,
}

impl<A, S> MyMcts<A, S>
    where S: State<A> {
    pub(crate) fn find_node() {
        unimplemented!()
    }
}


impl<A, S> Mcts<A, S> for MyMcts<A, S>
    where S: State<A>,
          A: Copy {
    fn new() -> MyMcts<A, S> {
        let mut arena = Arena::new();
        let root_node = arena.new_node(MctsStats::new(None));
        MyMcts {
            arena,
            root_node,
            root_state: S::initial(),
        }
    }


    fn best_play(&self, state: &S) -> A {
        let mut actions = state.actions();
        let mut rng = thread_rng();
        actions.shuffle(&mut rng);
        actions.get(0).unwrap().clone()
    }

    fn explore(&self, state: &mut S) {
        let mut i = 0;
        while state.result().is_none() {
            let a = self.best_play(state);
            state.next(&a);
            // state.prev();
            i += 1;
        }

        // while i > 0 {
        //     state.prev();
        //     i -= 1;
        // }
    }
}

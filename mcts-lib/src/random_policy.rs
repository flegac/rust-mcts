use std::cell::RefCell;
use std::ops::DerefMut;

use rand::prelude::SliceRandom;
use rand::SeedableRng;
use rand_pcg::Pcg64;

use policy::Policy;
use state::State;

pub struct RandomPolicy {
    rng: RefCell<Pcg64>,
}

impl RandomPolicy {
    pub fn new(seed: u64) -> RandomPolicy {
        RandomPolicy {
            rng: RefCell::new(Pcg64::seed_from_u64(seed)),
        }
    }
}

impl<A: Copy> Policy<A> for RandomPolicy {
    fn select<S: State<A>>(&self, state: &S) -> A {
        state.actions()
            .choose(self.rng.borrow_mut().deref_mut())
            .unwrap()
            .clone()
    }
}
use indextree::Arena;

use crate::gostate::GoAction;
use crate::mctsalgo::MctsAlgo;
use crate::state::{GameResult, State};

#[derive(Debug)]
pub struct Mcts<S> {
    arena: Arena<S>,
    root: S,
}


// impl<A, S> Mcts<S>
//     where S: State<A> {
//     pub fn explore(&mut self, root: NodeId, n: usize) -> NodeId {
//         if n == 0 {
//             root
//         } else {
//             let child = self.node(Some(root), S::new());
//             self.explore(child, n - 1)
//         }
//     }
//
//     pub fn node(&mut self, parent: Option<NodeId>, data: S) -> NodeId {
//         let child = { &mut self.arena }.new_node(data);
//         match parent {
//             None => {}
//             Some(p) => {
//                 p.append(child, &mut self.arena);
//             }
//         }
//         child
//     }
// }

impl<A, S> MctsAlgo<A, S> for Mcts<S>
    where S: State<A> {
    fn new() -> Mcts<S> {
        let arena = Arena::new();
        let root_state = S::new();
        // let root = { &arena }.new_node(root_state);
        Mcts {
            arena,
            root: root_state,
        }
    }

    fn root(&self) -> &S {
        &self.root
    }

    fn play(&self, first: &S) -> (GameResult, Vec<S>) {
        let a = &first.actions()[0];
        let s2 = first.next(&a);
        (GameResult::Victory, vec![s2])
    }
}

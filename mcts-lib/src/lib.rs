extern crate indextree;
extern crate rand;
extern crate rand_pcg;
extern crate graph_lib;
extern crate ordered_float;

pub mod mcts;
pub mod state;
pub mod mymcts;
mod mcts_stats;
pub mod policy;
pub mod random_policy;
pub mod mcts_state;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

extern crate indextree;
extern crate rand;
extern crate rand_pcg;
extern crate tree_lib;

pub mod mcts;
pub mod state;
pub mod mymcts;
mod stats;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

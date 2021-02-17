extern crate indextree;
extern crate rand;
extern crate rand_pcg;

pub mod mcts;
pub mod state;
pub mod mymcts;
mod stats;
mod trees;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

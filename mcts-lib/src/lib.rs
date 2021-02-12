extern crate indextree;
extern crate rand;

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

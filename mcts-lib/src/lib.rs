extern crate graph_lib;
extern crate indextree;
extern crate ordered_float;
extern crate rand;
extern crate rand_pcg;

pub mod mcts;
pub mod state;
pub mod mymcts;
pub mod policy;
pub mod random_policy;
mod mystate;
pub mod sim_result;

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;
    use std::time::{Duration, Instant};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_threads() {
        let (tx, rx) = mpsc::channel();
        let n = 1000;
        // let mut handles = vec![];

        for i in 0..n {
            let tx_copy = mpsc::Sender::clone(&tx);
            thread::spawn(move || {
                tx_copy.send(String::from(format!("thread {}", i))).unwrap();
            });
            // handles.push(h);
        }
        thread::spawn(move || {
            tx.send(String::from(format!("thread last",))).unwrap();
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }
}

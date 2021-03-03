extern crate itertools;
extern crate log;
extern crate env_logger;
extern crate chrono;

pub mod bench;
pub mod screen;
pub mod loggers;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::time::Duration;

pub struct Constants {
    pub full_time: Duration,
    pub round_time: Duration,
}


pub const GOBAN_SIZE: usize = 13;

pub const BENCH: Constants = Constants {
    full_time: Duration::from_secs(5),
    round_time: Duration::from_secs(1),
};
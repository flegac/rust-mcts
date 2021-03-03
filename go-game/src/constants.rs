use std::time::Duration;

use log::LevelFilter;

pub const SEED: u64 = 645;

pub const SIM_FACTOR: usize = 2;
pub const GOBAN_SIZE: usize = 13;
pub const LOG_LEVEL: LevelFilter = LevelFilter::Trace;
// pub const LOG_LEVEL: LevelFilter = LevelFilter::Info;
pub const BENCH: Constants = Constants {
    full_time: Duration::from_millis(1500),
    round_time: Duration::from_millis(500),
};

//-----------------------------------------------------------------------
pub struct Constants {
    pub full_time: Duration,
    pub round_time: Duration,
}
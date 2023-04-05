use std::time::Duration;

use log::LevelFilter;

pub const SEED: u64 = 645;

pub const SIM_FACTOR: usize = 1;
pub const GOBAN_SIZE: usize = 9;

// pub const LOG_LEVEL: LevelFilter = LevelFilter::Info;
// pub const LOG_LEVEL: LevelFilter = LevelFilter::Debug;
pub const LOG_LEVEL: LevelFilter = LevelFilter::Trace;

pub const BENCH: Constants = Constants {
    full_time: Duration::from_secs(2),
    round_time: Duration::from_millis(1000),
};

//-----------------------------------------------------------------------
pub struct Constants {
    pub full_time: Duration,
    pub round_time: Duration,
}
use std::fmt::{Display, Formatter};
use std::fmt;
use std::time::{Duration, Instant};

// TODO: use a struct for stats recording (in place of games attribute)
pub struct Bench {
    games: usize,
    start: Instant,
    time_limit: Duration,
    duration: Option<Duration>,
}

impl Bench {
    pub fn new(time_limit: Duration) -> Bench {
        Bench {
            games: 0,
            start: Instant::now(),
            time_limit,
            duration: None,
        }
    }

    pub fn inc_bench(&mut self, other: &Bench) {
        self.inc(other.games);
    }

    pub fn inc(&mut self, value: usize) {
        self.games += value;
    }

    pub fn looping(&mut self) -> bool {
        let res = self.start.elapsed() < self.time_limit;
        if !res && self.duration.is_none() {
            self.duration = Some(self.start.elapsed());
        }
        res
    }
}

impl Display for Bench {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Speed: {} games {:?} sec",
               self.games,
               self.duration.unwrap_or(self.start.elapsed())
        )
    }
}
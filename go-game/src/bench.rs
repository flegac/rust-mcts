use std::fmt::{Display, Formatter};
use std::fmt;
use std::time::{Duration, Instant};

pub struct Bench {
    iterations: usize,
    start: Instant,
    time_limit: Duration,
    duration: Option<Duration>,
}

impl Bench {
    pub fn new(time_limit: Duration) -> Bench {
        Bench {
            iterations: 0,
            start: Instant::now(),
            time_limit,
            duration: None,
        }
    }
    pub fn spawn(&mut self, time_limit: Duration) -> Bench {
        Bench {
            iterations: 0,
            start: Instant::now(),
            time_limit,
            duration: None,
        }
    }

    pub fn inc_bench(&mut self, other: &Bench) {
        self.inc(other.iterations);
    }

    pub fn inc(&mut self, value: usize) {
        self.iterations += value;
    }

    pub fn inc_easy(&mut self, other: Option<Bench>) {
        match other {
            None => self.inc(1),
            Some(o) => self.inc_bench(&o)
        }
    }


    pub fn looping_inc(&mut self, round: Option<Bench>) -> bool {
        let duration = self.start.elapsed();
        let finished = duration >= self.time_limit;
        match self.duration {
            None => match finished {
                true => self.duration = Some(duration),
                false => self.inc_easy(round)
            }
            Some(_) => {}
        }

        !finished
    }

    pub fn looping(&mut self) -> bool {
        let duration = self.start.elapsed();
        let finished = duration >= self.time_limit;
        match self.duration {
            None => if finished {
                self.duration = Some(duration);
            }
            Some(_) => {}
        }
        !finished
    }
}

impl Display for Bench {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Speed: {} iter {:?} sec",
               self.iterations,
               self.duration.unwrap_or(self.start.elapsed())
        )
    }
}
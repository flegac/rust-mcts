use std::fmt;
use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};

pub struct Bench {
    name: String,
    pub loops: usize,
    pub ops: usize,
    start: Instant,
    duration: Duration,
    speed_factor: Option<f32>,
}

impl Bench {
    pub fn new(name: &str) -> Bench {
        Bench {
            name: String::from(name),
            loops: 0,
            ops: 0,
            start: Instant::now(),
            duration: Duration::from_secs(0),
            speed_factor: None,
        }
    }

    pub fn with_speed(name: &str, speed_factor: f32) -> Bench {
        Bench {
            name: String::from(name),
            loops: 0,
            ops: 0,
            start: Instant::now(),
            duration: Duration::from_secs(0),
            speed_factor: Some(speed_factor),
        }
    }


    pub fn speed(&self) -> f32 {
        self.ops as f32 / self.duration.as_secs_f32()
    }

    pub fn inc_bench(&mut self, other: &Bench) {
        self.inc(other.ops);
    }

    pub fn inc(&mut self, value: usize) {
        self.ops += value;
    }

    pub fn for_iterations(&mut self, limit: usize) -> bool {
        self.until_condition(self.loops >= limit)
    }

    pub fn for_duration(&mut self, time_limit: Duration) -> bool {
        self.until_condition(self.duration >= time_limit)
    }

    pub fn until_condition(&mut self, finished: bool) -> bool {
        self.duration = self.start.elapsed();
        if !finished {
            self.loops += 1;
        }
        !finished
    }
}

impl Drop for Bench {
    fn drop(&mut self) {
        if self.ops == 0 {
            self.ops = self.loops;
        }
        println!("Bench: {}", self)
    }
}


impl Display for Bench {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let speed = self.speed() * self.speed_factor.unwrap_or(1.);
        write!(
            f,
            "{}\n\
            Speed: {} ops {:?}\n\
            {} op/sec\n\
            {} op/hour",
            self.name,
            self.ops,
            self.duration,
            (speed) as u32,
            (3600. * speed) as u32
        )
    }
}


#[test]
fn test_bench() {
    let mut cpt = 0;

    let mut bench = Bench::new("Testing Bench");
    let mut i = 0;
    while bench.for_duration(Duration::from_millis(60)) {
        let mut round = Bench::new(&format!("Round-{}", i));
        while round.for_duration(Duration::from_millis(60)) {
            cpt += 3;
            round.inc(1);
        }
        bench.inc_bench(&round);
        i += 1;
    }
}

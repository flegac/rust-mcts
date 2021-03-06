use std::fmt;
use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};

pub struct Bench {
    pub iterations: usize,
    start: Instant,
    duration: Duration,
    speed_factor: Option<f32>,
}

impl Bench {
    pub fn new() -> Bench {
        Bench {
            iterations: 0,
            start: Instant::now(),
            duration: Duration::from_secs(0),
            speed_factor: None,
        }
    }

    pub fn with_speed(speed_factor: f32) -> Bench {
        Bench {
            iterations: 0,
            start: Instant::now(),
            duration: Duration::from_secs(0),
            speed_factor: Some(speed_factor),
        }
    }


    pub fn speed(&self) -> f32 {
        self.iterations as f32 / self.duration.as_secs_f32()
    }

    pub fn inc_bench(&mut self, other: &Bench) {
        self.inc(other.iterations);
    }

    pub fn inc(&mut self, value: usize) {
        self.iterations += value;
    }

    pub fn for_iterations(&mut self, limit: usize) -> bool {
        self.until_condition(self.iterations >= limit)
    }

    pub fn for_duration(&mut self, time_limit: Duration) -> bool {
        self.until_condition(self.duration >= time_limit)
    }

    pub fn until_condition(&mut self, finished: bool) -> bool {
        self.duration = self.start.elapsed();
        if !finished {
            self.iterations += 1;
        }
        !finished
    }
}

impl Display for Bench {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let speed = self.speed() * self.speed_factor.unwrap_or(1.);
        write!(
            f,
            "Speed: {} iter {:?}\n\
            {} iter/sec\n\
            {} iter/min\n\
            {} iter/hour",
            self.iterations, self.duration,
            (speed) as u32,
            (60. * speed) as u32,
            (3600. * speed) as u32
        )
    }
}


#[test]
fn test_bench() {
    let mut cpt = 0;

    let mut bench = Bench::new();
    while bench.for_duration(Duration::from_millis(60)) {
        let mut round = Bench::new();
        while round.for_duration(Duration::from_millis(60)) {
            cpt += 3;
            round.inc(1);
        }
        println!("{}", round);
        bench.inc_bench(&round);
    }
    println!("{}\n{}", bench, bench.log_speed(1 as f32));
}

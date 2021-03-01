use policy::score::Score;
use sim_result::SimResult;

pub struct WinScore {}

impl WinScore {
    pub fn new() -> WinScore {
        WinScore {}
    }
}

impl Score for WinScore {
    fn score(&self, stats: &SimResult) -> f32 {
        match stats.tries {
            0 => 0.,
            n => {
                let w = stats.wins as f32;
                w / n as f32
            }
        }
    }
}

pub struct ExploreScore<'a> {
    parent: &'a SimResult,
}

impl<'a> ExploreScore<'a> {
    pub fn new(parent: &'a SimResult) -> ExploreScore {
        ExploreScore { parent }
    }
}

impl<'a> Score for ExploreScore<'a> {
    fn score(&self, stats: &SimResult) -> f32 {
        let total_tries = (self.parent.tries as f32).ln();
        let x = match stats.tries {
            0 => total_tries,
            n => 2. * total_tries / n as f32,
        };
        x.sqrt()
    }
}

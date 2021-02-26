use sim_result::SimResult;

pub trait Score where Self: Sized {
    fn score(&self, stats: &SimResult) -> f32;
}
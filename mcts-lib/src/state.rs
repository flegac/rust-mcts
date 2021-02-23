#[derive(Debug, Copy, Clone)]
pub enum GameResult {
    Victory,
    Defeat,
    Draw,
}

pub trait State<A> {
    fn reset(&mut self);
    fn result(&self) -> Option<GameResult>;
    fn actions(&self) -> Vec<A>;
    fn apply(&mut self, action: &A);
}

#[derive(Debug, Copy, Clone)]
pub enum GameResult {
    Victory,
    Defeat,
    Draw,
}

impl GameResult {
    pub fn switch(&self) -> Self {
        match self {
            GameResult::Victory => GameResult::Defeat,
            GameResult::Defeat => GameResult::Victory,
            GameResult::Draw => GameResult::Draw
        }
    }
}


pub trait State<A> {
    fn reset(&mut self);
    fn result(&self) -> Option<GameResult>;
    fn actions(&self) -> Vec<A>;
    fn apply(&mut self, action: &A);
}

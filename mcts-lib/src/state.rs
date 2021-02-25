#[derive(Debug, Copy, Clone)]
pub enum GameResult {
    Win,
    Lose,
    Draw,
}

impl GameResult {
    pub fn switch(&self) -> Self {
        match self {
            GameResult::Win => GameResult::Lose,
            GameResult::Lose => GameResult::Win,
            GameResult::Draw => GameResult::Draw
        }
    }
}


pub trait State<A> {
    fn reset(&mut self);
    fn result(&self) -> Option<GameResult>;
    fn actions(&self) -> Vec<A>;
    fn apply(&mut self, action: A);
}

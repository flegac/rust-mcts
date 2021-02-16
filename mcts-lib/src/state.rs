#[derive(Copy, Clone)]
pub enum GameResult {
    Victory,
    Defeat,
    Draw,
}

pub trait State<A> {
    fn initial() -> Self;

    fn result(&self) -> Option<GameResult>;

    fn actions(&self) -> Vec<A>;
    fn next(&mut self, action: &A);
}
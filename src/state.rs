pub enum GameResult {
    Victory,
    Defeat,
    Draw,
}

pub trait State<A>: Eq {
    fn new() -> Self;

    fn final_value(self) -> Option<GameResult>;
    fn eval(self) -> usize;

    fn actions(&self) -> Vec<A>;
    fn next(&self, action: &A) -> Self;

    // fn apply(&mut self, action: &A);
    // fn undo(&mut self, action: &A);
}

use stones::stone::Stone;

pub struct StoneScore {
    pub stone: Stone,
    pub territory: usize,
    pub captures: usize,
}

impl StoneScore {
    pub fn score(&self) -> usize {
        self.territory + self.captures
    }
}
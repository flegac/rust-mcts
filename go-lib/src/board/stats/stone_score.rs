use board::groups::stone::Stone;

#[derive(Copy, Clone)]
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
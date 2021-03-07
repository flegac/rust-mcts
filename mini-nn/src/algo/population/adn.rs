use std::fmt::{Debug, Display, Formatter};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Adn<T> {
    pub adn: T,
    pub score: f32,
}

impl<T> Adn<T> {
    pub fn new(adn: T) -> Self {
        Adn { adn, score: 0_f32 }
    }
}

impl<T: Display> Display for Adn<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("score: {}\n{}", self.score, self.adn))
    }
}
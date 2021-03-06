use crate::algo::adn::Adn;
use crate::algo::score::Score;

pub trait Agent<Env> {
    fn fitness(&self, env: &Env) -> Score;
}

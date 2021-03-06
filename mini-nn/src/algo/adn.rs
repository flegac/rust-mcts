use crate::algo::agent::Agent;
use crate::algo::score::Score;
use crate::algo::trainer::Trainer;

pub struct Population<T> {
    pub population: Vec<Adn<T>>,
}

impl<T: Agent<Env>, Env> Agent<Population<Env>> for Population<T> {
    fn fitness(&self, env_population: &Population<Env>) -> Score {
        let mut score = 0_f32;
        for mut adn in self.population.iter() {
            let mut adn_score = 0_f32;
            for env in env_population.population.iter() {
                let score = adn.adn.fitness(&env.adn);
                adn_score += score.value * score.value;
            }
            adn_score = adn_score.sqrt();
            score += adn_score * adn_score;
        }
        score = score.sqrt();

        Score { value: score }
    }
}


impl<T: Agent<Env>, Env> Trainer<T, Env> for Population<T> {
    fn fit(&mut self, dataset: &Vec<Env>) {
        for mut adn in self.population.iter_mut() {
            adn.score.value = 0_f32;
            for env in dataset.iter() {
                let score = adn.adn.fitness(env);
                adn.score.value += score.value * score.value;
            }
            adn.score.value = adn.score.value.sqrt();
        }
    }
}

#[derive(Debug, Clone)]
pub struct Adn<T> {
    pub adn: T,
    pub score: Score,
}

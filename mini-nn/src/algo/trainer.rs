use crate::algo::adn::{Adn, Population};
use crate::algo::agent::Agent;
use crate::algo::mutation::Mutation;
use crate::algo::score::Score;

pub trait Trainer<T, Env> {
    fn fit(&mut self, dataset: &Vec<Env>);
}

pub struct AlgoGen<T, M: Mutation<T>> {
    pub population: Population<T>,
    pub mutations: Population<M>,
}

impl<T: Agent<Env>, Env, M: Mutation<T> + Agent<Adn<T>>> Trainer<T, Env> for AlgoGen<T, M> {
    fn fit(&mut self, dataset: &Vec<Env>) {
        //compute scores for population
        self.population.fit(dataset);
        //compute scores for mutations
        self.mutations.fit(&self.population.population);
        todo!("select best mutations & populations")
    }
}

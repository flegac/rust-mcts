use crate::algo::mutations::mutation::Mutation;
use crate::algo::population::Population;

pub struct AlgoGen<T, M: Mutation<T>> {
    pub population: Population<T>,
    pub mutations: Population<M>,
}

// impl<T: Metric<Env>, Env, M: Mutation<T> + Metric<Adn<T>>> Trainer<T, Env> for AlgoGen<T, M> {
//     fn fit(&mut self, dataset: &Vec<Env>) {
//         //compute scores for population
//         self.population.fit(dataset);
//         //compute scores for mutations
//         // self.mutations.fit(&self.population.population);
//         // todo!("select best mutations & populations")
//     }
// }

pub trait Trainer<T> {
    fn fit(&mut self, dataset: &[T]);
}

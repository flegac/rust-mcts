use crate::algo::population::population::Population;

pub trait Mutation<T> {
    fn mutate(&self, adn: &mut T);
    fn mutate_pop(&self, population: &mut Population<T>) {
        for x in population.population.iter_mut() {
            self.mutate(&mut x.adn);
        }
    }
}

impl<T, M: Mutation<T>> Mutation<T> for Population<M> {
    fn mutate(&self, adn: &mut T) {
        for mutator in self.population.iter() {
            mutator.adn.mutate(adn);
        }
    }
}

use crate::algo::population::population::Population;

pub trait Mutation<Mod> {
    fn mutate(&self, adn: &mut Mod);
    fn mutate_pop(&self, population: &mut Population<Mod>) {
        for x in population.population.iter_mut() {
            self.mutate(&mut x.adn);
        }
    }
}

impl<Mod, Mut: Mutation<Mod>> Mutation<Mod> for Population<Mut> {
    fn mutate(&self, adn: &mut Mod) {
        for mutator in self.population.iter() {
            mutator.adn.mutate(adn);
        }
    }
}

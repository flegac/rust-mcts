use crate::algo::population::population::Population;

pub trait CrossOver<Mod> {
    fn cross(&self, m1: &mut Mod, m2: &mut Mod);
    fn cross_pop(&self, population: &mut Population<Mod>) {
        let size = population.population.len();
        for i in 1..(size / 2) {
            let (m1, m2) = &mut population.population.split_at_mut(i);
            self.cross(
                &mut m1[i-1].adn,
                &mut m2[m2.len() - 1 - i].adn,
            );
        }
    }
}


impl<Mod, Cr: CrossOver<Mod>> CrossOver<Mod> for Population<Cr> {
    fn cross(&self, m1: &mut Mod, m2: &mut Mod) {
        for mutator in self.population.iter() {
            mutator.adn.cross(m1, m2);
        }
    }
}

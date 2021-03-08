use crate::algo::population::adn::Adn;
use crate::algo::population::population::Population;
use crate::framework::model::Model;

pub struct GeneticModel<Mod> {
    pub population: Population<Mod>,
    pub best: Adn<Mod>,
}

impl<Mod> GeneticModel<Mod> {
    pub fn new<F: Fn() -> Mod>(init: F, size: usize) -> Self {
        let mut items = Vec::with_capacity(size);
        for _i in 0..size {
            items.push(init())
        }
        GeneticModel {
            population: Population::new(items),
            best: Adn::new(init()),
        }
    }
}

impl<X, Y, Mod: Model<X,Y>> Model<X, Y> for GeneticModel<Mod> {
    fn predict(&self, x: &X, y: &mut Y) {
        self.best.adn.predict(x, y);
    }
}

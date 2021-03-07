use crate::algo::population::adn::Adn;
use crate::algo::population::population::Population;
use crate::framework::model::Model;

pub struct GeneticModel<M> {
    pub population: Population<M>,
    pub best: Adn<M>,
}

impl<M> GeneticModel<M> {
    pub fn new<F: Fn() -> M>(init: F, size: usize) -> Self {
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

impl<X, Y, M: Model<X,Y>> Model<X, Y> for GeneticModel<M> {
    fn predict(&self, x: &X, y: &mut Y) {
        self.best.adn.predict(x, y);
    }
}

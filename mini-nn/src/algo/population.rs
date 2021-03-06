use std::fmt::{Display, Formatter};
use std::fmt;

use ordered_float::OrderedFloat;

use crate::algo::adn::Adn;

pub struct Population<T> {
    pub population: Vec<Adn<T>>,
}

impl<T> Population<T> {
    pub fn from_adn(population: Vec<Adn<T>>) -> Self {
        Population { population }
    }

    pub fn new(population: Vec<T>) -> Self {
        let mut adns = vec![];
        for x in population {
            adns.push(Adn::new(x));
        }
        Population { population: adns }
    }
    pub fn len(&self) -> usize {
        self.population.len()
    }

    pub fn best(&self) -> &Adn<T> {
        let mut best = &self.population[0];
        let mut min_score = OrderedFloat(best.score);
        for x in self.population.iter() {
            if OrderedFloat(x.score) < min_score {
                best = x;
                min_score = OrderedFloat(x.score);
            }
        }
        best
        // self.population
        //     .iter()
        //     .min_by_key(|x| OrderedFloat(x.score))
        //     .unwrap()
    }
}

impl<T: Display> Display for Population<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        for (i, x) in self.population.iter().enumerate() {
            if i != 0 {
                res.push('\n');
            }
            res.push_str(&x.to_string());
        }
        write!(f, "{}", res)
    }
}


use ordered_float::OrderedFloat;

use tensor_lib::tensor::Tensor;

use crate::algo::genetic_model::GeneticModel;
use crate::algo::mutations::conv_mut::ConvMut;
use crate::algo::mutation::Mutation;
use crate::algo::population::adn::Adn;
use crate::algo::population::population::Population;
use crate::framework::metric::Metric;
use crate::framework::metrics::mse::MSE;
use crate::framework::model::Model;
use crate::framework::trainer::Trainer;
use crate::algo::crossover::CrossOver;

pub struct GeneticTrainer<Mut, Cr> {
    pub mutations: Population<Mut>,
    pub crossovers: Population<Cr>
}

impl<Mut, Cr> GeneticTrainer<Mut, Cr> {
    pub fn new(mutations: Vec<Mut>, crossovers: Vec<Cr>) -> Self {
        GeneticTrainer {
            mutations: Population::new(mutations),
            crossovers: Population::new(crossovers),
        }
    }
}


impl<X, Mod, Mut, Cr> Trainer<X, Tensor, GeneticModel<Mod>> for GeneticTrainer<Mut, Cr>
    where
        Mod: Model<X, Tensor> + Clone,
        Mut: Mutation<Mod>,
        Cr: CrossOver<Mod>,

{
    fn fit(&self, model: &mut GeneticModel<Mod>, x: &Vec<X>, y: &Vec<Tensor>) {
        self.mutations.mutate_pop(&mut model.population);
        self.crossovers.cross_pop(&mut model.population);

        //update scores
        let mut pred = y.clone();
        model.best.adn.predict_map(x, &mut pred);
        let scores = MSE::score_zip(&pred, y);
        model.best.score = MSE::score_vec(&scores);
        // println!("{}", model.best.score);
        for m in model.population.population.iter_mut() {
            m.adn.predict_map(x, &mut pred);
            let scores = MSE::score_zip(&pred, y);
            m.score = MSE::score_vec(&scores);
        }

        //update best
        let new_best = model.population.best().clone();
        if new_best.score < model.best.score {
            model.best = new_best;
        }

        // update population to the fittests
        let old_ppop = model.population.len();
        let population_limit = model.population.len() -1;
        model.population.population.sort_by_key(|x| OrderedFloat(x.score));
        model.population.population.drain(..population_limit);

        while model.population.len() < old_ppop {
            model.population.population.push(Adn::new(model.best.adn.clone()));
        }
    }
}
use std::iter::FromIterator;

use log::LevelFilter;
use ordered_float::OrderedFloat;
use rand_distr::{Normal, NormalError};

use rust_tools::bench::Bench;
use rust_tools::loggers::init_logs;

use crate::algo::adn::Adn;
use crate::algo::mutations::add_mut::AddMut;
use crate::algo::mutations::mutation::Mutation;
use crate::algo::mutations::tensor_mutation::TensorMutation;
use crate::algo::population::Population;
use crate::algo::trainer::Trainer;
use crate::metrics::metric::Metric;
use crate::metrics::mse::MSE;
use crate::tensors::shape4::Shape4;
use crate::tensors::tensor::Tensor;

pub struct AlgoGen<T, M> {
    population: Population<T>,
    mutations: Population<M>,
    best: Adn<T>,
    mutation_power: f32,
}

impl<M: TensorMutation> AlgoGen<Tensor, M> {
    pub fn new(init: Tensor, size: usize, mutations: Vec<M>) -> Self {
        let mut items = Vec::with_capacity(size);
        for _i in 0..size {
            items.push(init.deep_clone())
        }
        AlgoGen {
            population: Population::new(items),
            mutations: Population::new(mutations),
            best: Adn::new(init),
            mutation_power: 0.1,
        }
    }
}

impl<M: TensorMutation> AlgoGen<Tensor, M> {
    fn update_score(&mut self, dataset: &[Tensor]) {
        let best_score = MSE::score_map(&self.best.adn, dataset);
        self.best.score = best_score;
        for mut x in self.population.population.iter_mut() {
            x.score = MSE::score_map(&x.adn, dataset);
        }
    }

    fn update_best(&mut self) {
        let new_best = self.population.best().clone();
        if new_best.score < self.best.score {
            self.best = new_best;
        }
    }

    pub fn select_best_population(&mut self) {
        let pop_size = self.population.len();
        self.keep_only(pop_size / 2);
        while self.population.len() < pop_size {
            self.population.population.push(Adn::new(self.best.adn.deep_clone()));
        }
    }

    pub fn keep_only(&mut self, population_limit: usize) {
        self.population.population.sort_by_key(|x| OrderedFloat(x.score));
        self.population.population.drain(..population_limit);
    }
}

impl<M: TensorMutation> Trainer<Tensor> for AlgoGen<Tensor, M> {
    fn fit(&mut self, dataset: &[Tensor]) {
        self.mutations.mutate_pop(&mut self.population);
        self.update_score(dataset);
        self.update_best();
        self.select_best_population();
    }
}

#[test]
fn test_trainer() -> Result<(), NormalError> {
    init_logs(LevelFilter::Trace);
    let dist = Normal::new(0.0, 1000.0)?;

    // CONFIG
    let shape = Shape4::vec3(28, 28, 1);
    let dataset_size = 1;

    // GENERATE DATASET
    let dataset = Vec::from_iter(
        (0..dataset_size)
            .map(|_| Tensor::from_distrib(shape, dist))
    );

    let mut mutations = vec![];
    for &x in &[0.01, 0.02] {
        mutations.push(AddMut::new(x));
        mutations.push(AddMut::new(-x));
        mutations.push(AddMut::new(-x));
    }

    let mut trainer = AlgoGen::new(
        Tensor::new(shape, 0_f32),
        10,
        mutations,
    );
    let mut bench = Bench::new();
    while bench.for_iterations(1_000) {
        trainer.fit(dataset.as_slice());
        if bench.iterations % 1 == 0 {
            log::info!("best: {} ", trainer.best, );
        }
    }

    log::info!("Final best: {}", trainer.best);
    // log::info!("Target dataset:\n");
    // for x in dataset {
    //     log::info!(" - {}", x);
    // }

    Ok(())
}

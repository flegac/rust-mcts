pub mod mutations;
pub mod population;
pub mod genetic_model;
pub mod genetic_trainer;
pub mod mutation;
pub mod crossover;
pub mod crossovers;

#[cfg(test)]
mod test {
    use std::borrow::Borrow;
    use std::iter::FromIterator;

    use log::LevelFilter;
    use rand_distr::NormalError;

    use rust_tools::bench::Bench;
    use rust_tools::loggers::init_logs;
    use tensor_lib::structs::shape4::Shape4;
    use tensor_lib::tensor::Tensor;

    use crate::algo::crossovers::conv_mut::ConvCross;
    use crate::algo::crossovers::mix_cross::MixCross;
    use crate::algo::genetic_model::GeneticModel;
    use crate::algo::genetic_trainer::GeneticTrainer;
    use crate::algo::mutations::add_mut::AddMut;
    use crate::algo::mutations::conv_mut::ConvMut;
    use crate::conv2::Conv2;
    use crate::framework::model::Model;
    use crate::framework::trainer::Trainer;

    #[test]
    fn test_trainer() -> Result<(), NormalError> {
        init_logs(LevelFilter::Trace);
        // CONFIG
        let x_shape = Shape4::vec3(10, 10, 1);
        let y_shape = Shape4::vec3(8, 8, 1);
        let dataset_size = 1;

        // GENERATE DATASET
        let xx = Vec::from_iter(
            (0..dataset_size).map(|_| Tensor::normal(x_shape.clone(), 0.0, 1000.0))
        );
        let yy = Vec::from_iter(
            (0..dataset_size).map(|_| Tensor::normal(y_shape.clone(), 0.0, 1000.0))
        );


        //framework
        let mut model = GeneticModel::new(
            || Conv2::new(
                3,
                x_shape.z().unwrap(),
                y_shape.z().unwrap(),
            ),
            100,
        );

        //2021-03-08T11:52:37 [INFO] - loop: 20 best: score: 6319.499

        //trainer
        let mut trainer: GeneticTrainer<ConvMut<AddMut>, ConvCross<MixCross>> = GeneticTrainer::new(
            vec![
                ConvMut::filter(AddMut::new(3.0)),
                ConvMut::filter(AddMut::new(-2.0)),
                ConvMut::bias(AddMut::new(3.0)),
                ConvMut::bias(AddMut::new(-2.0)),
            ],
            vec![
                ConvCross::filter(MixCross {}),
                ConvCross::bias(MixCross {})
            ],
        );


        let mut pred = yy[0].clone();
        let mut bench = Bench::new("Genetic algorithm");
        let mut last_best: f32 = 1_000_000.0;
        let mut stable_period = 0;
        let mut treshold = 1.0;
        while bench.for_iterations(100_000) {
            let mut round = Bench::new(&format!("Round-{}", bench.loops));
            while round.for_iterations(100) {
                trainer.fit(&mut model, &xx, &yy);
            }
            let delta = (model.best.score - last_best).abs();
            log::info!("loop: {} best: {} delta: {}", bench.loops, model.best, delta);
            if delta < treshold {
                if stable_period >= 2 {
                    for muta in trainer.mutations.population.iter_mut() {
                        let filter = match &muta.adn.filter {
                            None => { 0.0 }
                            Some(x) => x.power
                        };
                        let bias = match &muta.adn.bias {
                            None => { 0.0 }
                            Some(x) => x.power
                        };
                        muta.adn.filter.iter_mut().for_each(|x| x.power *= 0.5);
                        muta.adn.bias.iter_mut().for_each(|x| x.power *= 0.5);
                        log::info!("Reduce learning rate: bias={} filter={}", bias, filter);
                    }
                    treshold *= 0.5;
                    stable_period = 0;
                    log::info!("Reduce treshold: {}", treshold);
                }
                stable_period += 1;
            }

            // model.predict(&xx[0], &mut pred);
            // log::info!("expected: {}", &yy[0]);
            // log::info!("actual  : {}\n", &pred);
            last_best = last_best.min(model.best.score);
        }

        log::info!("Final best: {}", model.best);

        Ok(())
    }
}

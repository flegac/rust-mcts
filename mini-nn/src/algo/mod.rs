pub mod mutations;
pub mod population;
pub mod genetic_model;
pub mod genetic_trainer;

#[cfg(test)]
mod test {
    use std::iter::FromIterator;

    use log::LevelFilter;
    use rand_distr::NormalError;

    use rust_tools::bench::Bench;
    use rust_tools::loggers::init_logs;
    use tensor_lib::structs::shape4::Shape4;
    use tensor_lib::tensor::Tensor;

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
        let population_size = 10;
        let init = || Conv2::new(
            3,
            x_shape.z().unwrap(),
            y_shape.z().unwrap(),
        );
        let mut model = GeneticModel::new(init, population_size);

        //trainer
        let mutations = Vec::from_iter(
            [ 0.0001].iter().flat_map(|&power| vec![
                ConvMut::filter(AddMut::new(power)),
                ConvMut::filter(AddMut::new(-power)),
                ConvMut::bias(AddMut::new(power)),
                ConvMut::bias(AddMut::new(-power)),
            ].into_iter()
            )
        );
        let trainer = GeneticTrainer::new(mutations);


        let mut pred = yy[0].clone();
        let mut bench = Bench::new("Genetic algorithm");
        while bench.for_iterations(100_000) {
            trainer.fit(&mut model, &xx, &yy);
            if bench.loops % 500 == 0 {
                log::info!("best: {}", model.best.score);
                // model.predict(&xx[0], &mut pred);
                // log::info!("expected: {}", &yy[0]);
                // log::info!("actual  : {}\n", &pred);
            }
        }

        log::info!("Final best: {}", model.best);

        Ok(())
    }
}

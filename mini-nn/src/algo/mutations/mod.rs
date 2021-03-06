pub mod mutation;
pub mod add_mut;
pub mod mult_mut;
pub mod tensor_mutation;

#[cfg(test)]
mod tests {
    use crate::algo::mutations::add_mut::AddMut;
    use crate::algo::mutations::mutation::Mutation;
    use crate::algo::population::Population;
    use crate::tensors::shape4::Shape4;
    use crate::tensors::tensor::Tensor;

    #[test]
    fn test_mutate() {
        let shape = Shape4::vec1(5);
        let mut population = Population::new(vec![
            Tensor::new(shape, 0_f32),
        ]);
        let mutations = Population::new(vec![
            AddMut::new(-1_f32),
            AddMut::new(1_f32),
        ]);

        log::info!("{}", population);
        for i in 0..5 {
            mutations.mutate_pop(&mut population);
            log::info!("{}", population);
        }
    }
}
